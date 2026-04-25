use anyhow::Context;

#[derive(Debug)]
pub struct Project {
    name: String,
    project_type: String,
    version: String,
    guid: String,
    root_namespace: String,
    keyword: String,
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
    output_directory: String,
    // Requires interpolation:
    // IntermediateDirectory="$(SolutionDir)../binaries/$(PlatformName)/intermediates/$(ConfigurationName)/$(ProjectName)"
    intermediate_directory: String,
    configuraiton_type: u8, // ??
    character_set: u8,
    whole_program_optimization: bool,
    compiler_tool: CompilerTool,
    linker_tool: LinkerTool,
}

#[derive(Debug)]
pub struct Platform {
    name: String,
}

#[derive(Debug)]
pub struct CompilerTool {
    additional_options: String,
    optimization: u8,
    inline_function_expansion: u8,
    enable_intrinsic_functions: bool,
    omit_frame_pointers: bool,
    enable_fiber_safe_optimizations: bool,
    // Requires interpolation: $(SolutionDir)/stlport;
    additional_include_directories: Vec<String>,
    // PreprocessorDefinitions="WIN32;NDEBUG;VOSTOK_STATIC_LIBRARIES;MASTER_GOLD;"
    preprocessor_definitions: Vec<String>,
    string_pooling: bool,
    minimal_rebuild: bool,
    exception_handling: u8,
    runtime_library: u8,
    buffer_security_check: bool,
    enable_enhanced_instruction_set: u8,
    floating_point_model: u8,
    use_precompiled_header: u8,
    precompiled_header_through: String,
    warning_level: u8,
    detect_64bit_portability_problems: bool,
    debug_information_format: u8,
}

#[derive(Debug)]
pub struct LinkerTool {
    additional_options: String,
    // Requires interpolation: $(SolutionDir)../binaries/$(PlatformName)/libraries/vostok_$(ProjectName)-static-gold.lib"
    output_file: String,
}

#[derive(Debug, Default)]
pub struct Files {
    filters: Vec<Filter>,
    files: Vec<File>,
}

#[derive(Debug)]
pub struct Filter {
    name: String,
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
        $(let mut $field_opt: Option<&str> = None;)*
        $(let mut $field: Option<&str> = None;)*

        for attr in $node.attributes() {
            match attr.name() {
                $($ignore)|*|"" => {}
                $($attr_name_opt => _ = $field_opt.replace(attr.value()),)*
                $($attr_name => _ = $field.replace(attr.value()),)*
                attr_name => anyhow::bail!("Unexpected {} attribute: '{attr_name}'", $ctx),
            }
        }

        $(let $field = $field.context(concat!($ctx, " missing '", $attr_name, "'"))?;)*
    };
}

impl Project {
    pub fn parse_xml(input: &str) -> anyhow::Result<Self> {
        let xml = roxmltree::Document::parse(input)?;
        let root = xml.root_element();
        if root.tag_name().name() != "VisualStudioProject" {
            anyhow::bail!("Expected 'VisualStudioProject' as a root object")
        }

        parse_attrs!(root, "VisualStudioProject", {
            "ProjectType"            => project_type,
            "Version"                => version,
            "Name"                   => name,
            "ProjectGUID"            => guid,
            "RootNamespace"          => root_namespace,
            "Keyword"                => keyword,
            "TargetFrameworkVersion" => target_framework_version,
        });

        let project_type = project_type.to_string();
        let version = version.to_string();
        let name = name.to_string();
        let guid = guid.to_string();
        let root_namespace = root_namespace.to_string();
        let keyword = keyword.to_string();
        let target_framework_version = target_framework_version.to_string();

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
                "ToolFiles" | "References" | "Globals" => {
                    if child.children().any(|n| n.is_element()) {
                        anyhow::bail!("Expected '{}' to be empty", child.tag_name().name());
                    }
                }
                "" => {}
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
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "Configuration", {
            "Name"                     => name,
            "OutputDirectory"          => output_directory,
            "IntermediateDirectory"    => intermediate_directory,
            "ConfigurationType"        => configuraiton_type,
            "CharacterSet"             => character_set,
            "WholeProgramOptimization" => whole_program_optimization,
        });

        let name = name.to_string();
        let output_directory = output_directory.to_string();
        let intermediate_directory = intermediate_directory.to_string();
        let configuraiton_type = configuraiton_type.parse()?;
        let character_set = character_set.parse()?;
        let whole_program_optimization = parse_bool(whole_program_optimization)?;

        let mut compiler_tool = None;
        let mut linker_tool = None;
        for child in node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "Tool")
        {
            match child.attribute("Name") {
                Some("VCCLCompilerTool") => compiler_tool = Some(CompilerTool::parse_xml(child)?),
                Some("VCLibrarianTool") => linker_tool = Some(LinkerTool::parse_xml(child)?),
                _ => (),
            }
        }
        let compiler_tool = compiler_tool.context("Configuration missing VCCLCompilerTool")?;
        let linker_tool = linker_tool.context("Configuration missing VCLibrarianTool")?;

        Ok(Self {
            name,
            output_directory,
            intermediate_directory,
            configuraiton_type,
            character_set,
            whole_program_optimization,
            compiler_tool,
            linker_tool,
        })
    }
}

impl CompilerTool {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "VCCLCompilerTool", {
            ignore: "Name",
            "AdditionalOptions"              => additional_options,
            "Optimization"                   => optimization,
            "InlineFunctionExpansion"        => inline_function_expansion,
            "EnableIntrinsicFunctions"       => enable_intrinsic_functions,
            "OmitFramePointers"              => omit_frame_pointers,
            "EnableFiberSafeOptimizations"   => enable_fiber_safe_optimizations,
            "AdditionalIncludeDirectories"   => additional_include_directories,
            "PreprocessorDefinitions"        => preprocessor_definitions,
            "StringPooling"                  => string_pooling,
            "MinimalRebuild"                 => minimal_rebuild,
            "ExceptionHandling"              => exception_handling,
            "RuntimeLibrary"                 => runtime_library,
            "BufferSecurityCheck"            => buffer_security_check,
            "EnableEnhancedInstructionSet"   => enable_enhanced_instruction_set,
            "FloatingPointModel"             => floating_point_model,
            "UsePrecompiledHeader"           => use_precompiled_header,
            "PrecompiledHeaderThrough"       => precompiled_header_through,
            "WarningLevel"                   => warning_level,
            "Detect64BitPortabilityProblems" => detect_64bit_portability_problems,
            "DebugInformationFormat"         => debug_information_format,
        });

        let additional_options = additional_options.to_string();
        let optimization = optimization.parse()?;
        let inline_function_expansion = inline_function_expansion.parse()?;
        let enable_intrinsic_functions = parse_bool(enable_intrinsic_functions)?;
        let omit_frame_pointers = parse_bool(omit_frame_pointers)?;
        let enable_fiber_safe_optimizations = parse_bool(enable_fiber_safe_optimizations)?;
        let additional_include_directories = parse_list(additional_include_directories);
        let preprocessor_definitions = parse_list(preprocessor_definitions);
        let string_pooling = parse_bool(string_pooling)?;
        let minimal_rebuild = parse_bool(minimal_rebuild)?;
        let exception_handling = exception_handling.parse()?;
        let runtime_library = runtime_library.parse()?;
        let buffer_security_check = parse_bool(buffer_security_check)?;
        let enable_enhanced_instruction_set = enable_enhanced_instruction_set.parse()?;
        let floating_point_model = floating_point_model.parse()?;
        let use_precompiled_header = use_precompiled_header.parse()?;
        let precompiled_header_through = precompiled_header_through.to_string();
        let warning_level = warning_level.parse()?;
        let detect_64bit_portability_problems = parse_bool(detect_64bit_portability_problems)?;
        let debug_information_format = debug_information_format.parse()?;

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
        })
    }
}

impl LinkerTool {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "VCLibrarianTool", {
            ignore: "Name",
            "AdditionalOptions" => additional_options,
            "OutputFile"        => output_file,
        });

        let additional_options = additional_options.to_string();
        let output_file = output_file.to_string();

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
                tag => anyhow::bail!("Unexpected tag in Files: '{tag}'"),
            }
        }

        Ok(Self { filters, files })
    }
}

impl Filter {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "Filter", {
            "Name" => name,
        });

        let name = name.to_string();

        let mut filters = vec![];
        let mut files = vec![];

        for child in node.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "Filter" => filters.push(Filter::parse_xml(child)?),
                "File" => files.push(File::parse_xml(child)?),
                tag => anyhow::bail!("Unexpected tag in Filter: '{tag}'"),
            }
        }

        Ok(Self {
            name,
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
                tag => anyhow::bail!("Unexpected tag in File: '{tag}'"),
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
        _ => anyhow::bail!("Unexpected boolean value: {s}"),
    }
}

fn parse_list(s: &str) -> Vec<String> {
    s.split(';')
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}
