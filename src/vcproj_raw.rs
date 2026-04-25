use anyhow::Context;

#[derive(Debug)]
pub struct VCProject {
    name: String,
    project_type: String,
    version: String,
    guid: String,
    root_namespace: String,
    keyword: Option<String>, // TODO: freeimage\LibOpenJPEG\LibOpenJPEG.vcproj
    target_framework_version: String,
    platforms: Vec<Platform>,
    configuraitons: Vec<Configuration>,
    files: Files,
}

#[derive(Debug)]
pub struct Configuration {
    name: String,
    // Requires interpolation:
    // OutputDirectory="$(SolutionDir)../binaries/$(PlatformName)/intermediates/$(ConfigurationName)/$(ProjectName)"
    output_directory: Option<String>, // TODO: Can be missing in game in random config
    // Requires interpolation:
    // IntermediateDirectory="$(SolutionDir)../binaries/$(PlatformName)/intermediates/$(ConfigurationName)/$(ProjectName)"
    intermediate_directory: Option<String>, // TODO: Same as above
    configuraiton_type: u8,                 // ??
    character_set: Option<u8>,
    whole_program_optimization: bool,
    managed_extensions: Option<u8>,

    compiler_tool: Option<CompilerTool>, // TODO: Can be missing for Xbox
    lib_tool: Option<LibTool>,
    linker_tool: Option<LinkerTool>,
}

#[derive(Debug)]
pub struct Platform {
    name: String,
}

#[derive(Debug)]
#[rustfmt::skip]
pub struct CompilerTool {
    additional_options:                Option<String>,
    optimization:                      Option<u8>,
    inline_function_expansion:         Option<u8>,
    enable_intrinsic_functions:        Option<bool>,
    omit_frame_pointers:               Option<bool>,
    enable_fiber_safe_optimizations:   Option<bool>,
    // Requires interpolation: $(SolutionDir)/stlport;
    additional_include_directories:    Option<Vec<String>>,
    // PreprocessorDefinitions="WIN32;NDEBUG;VOSTOK_STATIC_LIBRARIES;MASTER_GOLD;"
    preprocessor_definitions:          Option<Vec<String>>,
    string_pooling:                    Option<bool>,
    minimal_rebuild:                   Option<bool>,
    exception_handling:                Option<u8>,
    runtime_library:                   Option<u8>,
    buffer_security_check:             Option<bool>,
    enable_enhanced_instruction_set:   Option<u8>,
    floating_point_model:              Option<u8>,
    use_precompiled_header:            Option<u8>,
    precompiled_header_through:        Option<String>,
    warning_level:                     Option<u8>,
    detect_64bit_portability_problems: Option<bool>,
    debug_information_format:          Option<u8>,
    basic_runtime_checks:              Option<u8>,
    favor_size_or_speed:               Option<u8>,
    enable_function_level_linking:     Option<bool>,
    precompiled_header_file:           Option<String>,
    whole_program_optimization:        Option<bool>,
    smaller_type_check:                Option<bool>,
}

#[derive(Debug)]
#[rustfmt::skip]
pub struct LinkerTool {
    additional_dependencies:        Option<Vec<String>>,
    output_file:                    Option<String>,
    link_incremental:               Option<u8>,
    additional_library_directories: Option<Vec<String>>,
    ignore_default_library_names:   Option<Vec<String>>,
    module_definition_file:         Option<String>,
    generate_debug_information:     Option<bool>,
    program_database_file:          Option<String>,
    generate_map_file:              Option<bool>,
    map_file_name:                  Option<String>,
    map_exports:                    Option<bool>,
    sub_system:                     Option<u8>,
    large_address_aware:            Option<u8>,
    optimize_references:            Option<u8>,
    enable_comdat_folding:          Option<u8>,
    randomized_base_address:        Option<u8>,
    data_execution_prevention:      Option<u8>,
    import_library:                 Option<String>,
    target_machine:                 Option<u8>,
}

#[derive(Debug)]
pub struct LibTool {
    additional_options: Option<String>,
    // Requires interpolation: $(SolutionDir)../binaries/$(PlatformName)/libraries/vostok_$(ProjectName)-static-gold.lib"
    output_file: Option<String>, // TODO: nvidia\nvt\project\squish.vcproj
}

#[derive(Debug, Default)]
pub struct Files {
    filters: Vec<Filter>,
    files: Vec<File>,
}

#[derive(Debug)]
pub struct Filter {
    name: String,
    filter: Option<Vec<String>>,
    filters: Vec<Filter>,
    files: Vec<File>,
}

#[derive(Debug)]
pub struct File {
    relative_path: String,
    file_configurations: Vec<FileConfiguration>,
}

#[derive(Debug)]
pub struct FileConfiguration {
    name: String,
    excluded_from_build: bool,
    tool: Tool,
}

#[derive(Debug)]
pub struct Tool {
    name: String,
    use_precompiled_header: Option<u8>,
}

macro_rules! parse_attrs {
    ($node:expr, $ctx:literal, {
        $(ignore: $ignore:literal,)*
        $(optional: $attr_name_opt:literal => $field_opt:ident,)*
        $($attr_name:literal => $field:ident,)*
    }) => {
        $(let mut $field: Option<&str> = None;)*
        $(let mut $field_opt: Option<&str> = None;)*

        for attr in $node.attributes() {
            match attr.name() {
                $($ignore)|*|"" => {}
                $($attr_name => _ = $field.replace(attr.value()),)*
                $($attr_name_opt => _ = $field_opt.replace(attr.value()),)*
                attr_name => {
                    eprintln!("Unexpected {} attribute: '{attr_name}' with value: '{:?}'", $ctx, attr.value());
                    // anyhow::bail!("Unexpected {} attribute: '{attr_name}' with value: '{}'", $ctx, attr.value()),
                }
            }
        }

        $(let $field = $field.context(concat!($ctx, " missing '", $attr_name, "'"))?;)*
    };
}

#[rustfmt::skip]
macro_rules! optparse {
    ($f:ident: bool)   => { let $f = $f.map(parse_bool).transpose()?; };
    ($f:ident: String) => { let $f = $f.map(str::to_string); };
    ($f:ident: Vec<_>) => { let $f = $f.map(parse_list); };
    ($f:ident: $t:ty)  => { let $f = $f.map(|s| s.parse::<$t>()).transpose()?; };
}

impl VCProject {
    #[rustfmt::skip]
    pub fn parse_xml(input: &str) -> anyhow::Result<Self> {
        let xml = roxmltree::Document::parse(input)?;
        let root = xml.root_element();
        if root.tag_name().name() != "VisualStudioProject" {
            anyhow::bail!("Expected 'VisualStudioProject' as a root object")
        }

        parse_attrs!(root, "VisualStudioProject", {
            optional: "Keyword"                => keyword,
            "ProjectType"            => project_type,
            "Version"                => version,
            "Name"                   => name,
            "ProjectGUID"            => guid,
            "RootNamespace"          => root_namespace,
            "TargetFrameworkVersion" => target_framework_version,
        });

        let project_type             = project_type.to_string();
        let version                  = version.to_string();
        let name                     = name.to_string();
        let guid                     = guid.to_string();
        let root_namespace           = root_namespace.to_string();
        let target_framework_version = target_framework_version.to_string();
        optparse!(keyword: String);

        let mut platforms: Vec<Platform> = vec![];
        let mut configuraitons: Vec<Configuration> = vec![];
        let mut files: Files = Files::default();

        for child in root.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "Platforms" => {
                    for node in child
                        .children()
                        .filter(|n| n.is_element() && n.tag_name().name() == "Platform")
                    {
                        platforms.push(Platform::parse_xml(node)?);
                    }
                }
                "Configurations" => {
                    for node in child
                        .children()
                        .filter(|n| n.is_element() && n.tag_name().name() == "Configuration")
                    {
                        configuraitons.push(Configuration::parse_xml(node)?);
                    }
                }
                "Files" => {
                    files = Files::parse_xml(child)?;
                }
                "ToolFiles" => {
                    if child.children().any(|n| n.is_element()) {
                        anyhow::bail!("Expected '{}' to be empty", child.tag_name().name());
                    }
                }
                "References" => {
                    // TODO: BugTrapN.vcproj
                    // <References>
                    // 	<AssemblyReference
                    // 		RelativePath="System.dll"
                    // 		AssemblyName="System, Version=2.0.0.0, PublicKeyToken=b77a5c561934e089, processorArchitecture=MSIL"
                    // 		MinFrameworkVersion="131072"
                    // 	/>
                    // 	<AssemblyReference
                    // 		RelativePath="System.Windows.Forms.dll"
                    // 		AssemblyName="System.Windows.Forms, Version=2.0.0.0, PublicKeyToken=b77a5c561934e089, processorArchitecture=MSIL"
                    // 		MinFrameworkVersion="131072"
                    // 	/>
                    // </References>
                }
                "Globals" => {
                    // TODO:  'ode\sources\ode.vcproj'
                    // <Globals>
                    // 	<Global
                    // 		Name="DevPartner_IsInstrumented"
                    // 		Value="0"
                    // 	/>
                    // </Globals>
                }
                "" => (),
                tag_name => anyhow::bail!("Unexpected tag name: '{tag_name}'"),
            }
        }

        Ok(Self {
            name,
            project_type,
            version,
            guid,
            root_namespace,
            keyword,
            target_framework_version,
            platforms,
            configuraitons,
            files,
        })
    }
}

impl Platform {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "Platform", {
            "Name" => name,
        });

        let name = name.to_string();

        Ok(Self { name })
    }
}

impl Configuration {
    #[rustfmt::skip]
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "Configuration", {
            optional: "WholeProgramOptimization" => whole_program_optimization,
            optional: "ManagedExtensions" => managed_extensions,
            optional: "CharacterSet"      => character_set,
            optional: "OutputDirectory"          => output_directory,
            optional: "IntermediateDirectory"    => intermediate_directory,
            "Name"                     => name,
            "ConfigurationType"        => configuraiton_type,
        });

        let name                       = name.to_string();
        let configuraiton_type         = configuraiton_type.parse::<u8>()?;
        let whole_program_optimization = parse_bool(whole_program_optimization.unwrap_or("false"))?;
        optparse!(managed_extensions: u8);
        optparse!(character_set: u8);
        optparse!(output_directory: String);
        optparse!(intermediate_directory: String);

        let mut compiler_tool = None;
        let mut lib_tool = None;
        let mut linker_tool = None;

        for child in node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "Tool")
        {
            match child.attribute("Name") {
                // TODO
                Some("VCCLCompilerTool") => compiler_tool = Some(CompilerTool::parse_xml(child)?),
                Some("VCLibrarianTool") => lib_tool = Some(LibTool::parse_xml(child)?),
                Some("VCLinkerTool") => linker_tool = Some(LinkerTool::parse_xml(child)?),
                _ => (),
            }
        }


        Ok(Self {
            name,
            output_directory,
            intermediate_directory,
            configuraiton_type,
            character_set,
            whole_program_optimization,
            managed_extensions,

            compiler_tool,
            lib_tool,
            linker_tool,
        })
    }
}

impl CompilerTool {
    #[rustfmt::skip]
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "VCCLCompilerTool", {
            ignore: "Name",
            optional: "AdditionalOptions"              => additional_options,
            optional: "Optimization"                   => optimization,
            optional: "InlineFunctionExpansion"        => inline_function_expansion,
            optional: "EnableIntrinsicFunctions"       => enable_intrinsic_functions,
            optional: "OmitFramePointers"              => omit_frame_pointers,
            optional: "EnableFiberSafeOptimizations"   => enable_fiber_safe_optimizations,
            optional: "AdditionalIncludeDirectories"   => additional_include_directories,
            optional: "PreprocessorDefinitions"        => preprocessor_definitions,
            optional: "StringPooling"                  => string_pooling,
            optional: "MinimalRebuild"                 => minimal_rebuild,
            optional: "ExceptionHandling"              => exception_handling,
            optional: "RuntimeLibrary"                 => runtime_library,
            optional: "BufferSecurityCheck"            => buffer_security_check,
            optional: "EnableEnhancedInstructionSet"   => enable_enhanced_instruction_set,
            optional: "FloatingPointModel"             => floating_point_model,
            optional: "UsePrecompiledHeader"           => use_precompiled_header,
            optional: "PrecompiledHeaderThrough"       => precompiled_header_through,
            optional: "WarningLevel"                   => warning_level,
            optional: "Detect64BitPortabilityProblems" => detect_64bit_portability_problems,
            optional: "DebugInformationFormat"         => debug_information_format,
            optional: "BasicRuntimeChecks"             => basic_runtime_checks,
            optional: "FavorSizeOrSpeed"               => favor_size_or_speed,
            optional: "EnableFunctionLevelLinking"     => enable_function_level_linking,
            optional: "PrecompiledHeaderFile"          => precompiled_header_file,
            optional: "WholeProgramOptimization"       => whole_program_optimization,
            optional: "SmallerTypeCheck"             => smaller_type_check,
        });

        optparse!(additional_options:                String);
        optparse!(optimization:                      u8);
        optparse!(inline_function_expansion:         u8);
        optparse!(enable_intrinsic_functions:        bool);
        optparse!(omit_frame_pointers:               bool);
        optparse!(enable_fiber_safe_optimizations:   bool);
        optparse!(additional_include_directories:    Vec<_>);
        optparse!(preprocessor_definitions:          Vec<_>);
        optparse!(string_pooling:                    bool);
        optparse!(minimal_rebuild:                   bool);
        optparse!(exception_handling:                u8);
        optparse!(runtime_library:                   u8);
        optparse!(buffer_security_check:             bool);
        optparse!(enable_enhanced_instruction_set:   u8);
        optparse!(floating_point_model:              u8);
        optparse!(use_precompiled_header:            u8);
        optparse!(precompiled_header_through:        String);
        optparse!(warning_level:                     u8);
        optparse!(detect_64bit_portability_problems: bool);
        optparse!(debug_information_format:          u8);
        optparse!(basic_runtime_checks:              u8);
        optparse!(favor_size_or_speed:               u8);
        optparse!(enable_function_level_linking:     bool);
        optparse!(precompiled_header_file:           String);
        optparse!(whole_program_optimization:        bool);
        optparse!(smaller_type_check:        bool);



        Ok(Self {
            additional_options,
            optimization,
            inline_function_expansion,
            enable_intrinsic_functions,
            omit_frame_pointers,
            enable_fiber_safe_optimizations,
            additional_include_directories,
            preprocessor_definitions,
            string_pooling,
            minimal_rebuild,
            exception_handling,
            runtime_library,
            buffer_security_check,
            enable_enhanced_instruction_set,
            floating_point_model,
            use_precompiled_header,
            precompiled_header_through,
            warning_level,
            detect_64bit_portability_problems,
            debug_information_format,
            basic_runtime_checks,
            favor_size_or_speed,
            enable_function_level_linking,
            precompiled_header_file,
            whole_program_optimization,
            smaller_type_check,
        })
    }
}

impl LinkerTool {
    #[rustfmt::skip]
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "VCLinkerTool", {
            ignore: "Name",
            optional: "AdditionalDependencies"        => additional_dependencies,
            optional: "OutputFile"                    => output_file,
            optional: "LinkIncremental"               => link_incremental,
            optional: "AdditionalLibraryDirectories"  => additional_library_directories,
            optional: "IgnoreDefaultLibraryNames"     => ignore_default_library_names,
            optional: "ModuleDefinitionFile"          => module_definition_file,
            optional: "GenerateDebugInformation"      => generate_debug_information,
            optional: "ProgramDatabaseFile"           => program_database_file,
            optional: "GenerateMapFile"               => generate_map_file,
            optional: "MapFileName"                   => map_file_name,
            optional: "MapExports"                    => map_exports,
            optional: "SubSystem"                     => sub_system,
            optional: "LargeAddressAware"             => large_address_aware,
            optional: "OptimizeReferences"            => optimize_references,
            optional: "EnableCOMDATFolding"           => enable_comdat_folding,
            optional: "RandomizedBaseAddress"         => randomized_base_address,
            optional: "DataExecutionPrevention"       => data_execution_prevention,
            optional: "ImportLibrary"                 => import_library,
            optional: "TargetMachine"                 => target_machine,
        });

        optparse!(additional_dependencies: Vec<_>);
        optparse!(output_file: String);
        optparse!(link_incremental: u8);
        optparse!(additional_library_directories: Vec<_>);
        optparse!(ignore_default_library_names: Vec<_>);
        optparse!(module_definition_file: String);
        optparse!(generate_debug_information: bool);
        optparse!(program_database_file: String);
        optparse!(generate_map_file: bool);
        optparse!(map_file_name: String);
        optparse!(map_exports: bool);
        optparse!(sub_system: u8);
        optparse!(large_address_aware: u8);
        optparse!(optimize_references: u8);
        optparse!(enable_comdat_folding: u8);
        optparse!(randomized_base_address: u8);
        optparse!(data_execution_prevention: u8);
        optparse!(import_library: String);
        optparse!(target_machine: u8);

        Ok(Self {
            additional_dependencies,
            output_file,
            link_incremental,
            additional_library_directories,
            ignore_default_library_names,
            module_definition_file,
            generate_debug_information,
            program_database_file,
            generate_map_file,
            map_file_name,
            map_exports,
            sub_system,
            large_address_aware,
            optimize_references,
            enable_comdat_folding,
            randomized_base_address,
            data_execution_prevention,
            import_library,
            target_machine,
        })
    }
}

impl LibTool {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "VCLibrarianTool", {
            ignore: "Name",
            optional: "AdditionalOptions" => additional_options,
            optional: "OutputFile"        => output_file,
        });

        optparse!(additional_options: String);
        optparse!(output_file: String);

        Ok(Self {
            additional_options,
            output_file,
        })
    }
}

impl Files {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        let mut filters = vec![];
        let mut files = vec![];

        for child in node.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "Filter" => filters.push(Filter::parse_xml(child)?),
                "File" => files.push(File::parse_xml(child)?),
                tag => eprintln!("Unexpected tag in Files: '{tag}'"),
                // tag => anyhow::bail!("Unexpected tag in Files: '{tag}'"),
            }
        }

        Ok(Self { filters, files })
    }
}

impl Filter {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "Filter", {
            optional: "Filter" => filter,
            "Name" => name,
        });

        let name = name.to_string();
        optparse!(filter: Vec<_>);

        let mut filters = vec![];
        let mut files = vec![];

        for child in node.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "Filter" => filters.push(Filter::parse_xml(child)?),
                "File" => files.push(File::parse_xml(child)?),
                tag => eprintln!("Unexpected tag in Filter: '{tag}'"),
                // tag => anyhow::bail!("Unexpected tag in Filter: '{tag}'"),
            }
        }

        Ok(Self {
            name,
            filter,

            filters,
            files,
        })
    }
}

impl File {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "File", {
            "RelativePath" => relative_path,
        });

        let relative_path = relative_path.to_string();

        let mut file_configurations = vec![];

        for child in node.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "FileConfiguration" => {
                    let file_configuration = FileConfiguration::parse_xml(child)?;
                    if file_configuration.name.contains("PS3")
                        || file_configuration.name.contains("Xbox")
                        || file_configuration.name.contains("x64")
                    {
                        continue;
                    }

                    file_configurations.push(file_configuration)
                }
                "Tool" => {}
                tag => eprintln!("Unexpected tag in File: '{tag}'"),
                // tag => anyhow::bail!("Unexpected tag in File: '{tag}'"),
            }
        }

        Ok(Self {
            relative_path,
            file_configurations,
        })
    }
}

impl FileConfiguration {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "FileConfiguration", {
            optional: "ExcludedFromBuild" => excluded_from_build,
                       "Name"             => name,
        });

        let name = name.to_string();
        let excluded_from_build = parse_bool(excluded_from_build.unwrap_or("false"))?;

        let tool = node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "Tool")
            .next()
            .context("FileConfiguration missing Tool")
            .and_then(Tool::parse_xml)?;

        Ok(Self {
            name,
            excluded_from_build,
            tool,
        })
    }
}

impl Tool {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "Tool", {
            optional: "UsePrecompiledHeader" => use_precompiled_header,
                      "Name"                 => name,
        });

        let name = name.to_string();
        let use_precompiled_header = use_precompiled_header.map(str::parse::<u8>).transpose()?;

        Ok(Self {
            name,
            use_precompiled_header,
        })
    }
}

fn parse_bool(s: &str) -> anyhow::Result<bool> {
    match s {
        "1" | "TRUE" | "true" => Ok(true),
        "0" | "FALSE" | "false" => Ok(false),
        _ => anyhow::bail!("Unexpected boolean value: '{s}'"),
    }
}

fn parse_list(s: &str) -> Vec<String> {
    s.split(';')
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}
