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
#[rustfmt::skip]
pub struct Configuration {
    name:                                  String,
    // Requires interpolation:
    // OutputDirectory="$(SolutionDir)../binaries/$(PlatformName)/intermediates/$(ConfigurationName)/$(ProjectName)"
    output_directory:                      Option<String>, // TODO: Can be missing in game in random config
    // Requires interpolation:
    // IntermediateDirectory="$(SolutionDir)../binaries/$(PlatformName)/intermediates/$(ConfigurationName)/$(ProjectName)"
    intermediate_directory:                Option<String>, // TODO: Same as above
    configuraiton_type:                    u8,                 // ??
    character_set:                         Option<u8>,
    whole_program_optimization:            bool,
    managed_extensions:                    Option<u8>,
    atl_minimizes_c_runtime_library_usage: Option<bool>,
    delete_extensions_on_clean:            Option<String>,
    inherited_property_sheets:             Option<Vec<String>>,
    use_of_atl:                            Option<u8>,
    use_of_mfc:                            Option<i8>,

    compiler_tool:                         Option<CompilerTool>, // TODO: Can be missing for Xbox
    lib_tool:                              Option<LibTool>,
    linker_tool:                           Option<LinkerTool>,
}

#[derive(Debug)]
pub struct Platform {
    name: String,
}

#[derive(Debug)]
#[rustfmt::skip]
pub struct CompilerTool {
    additional_options:                  Option<String>,
    optimization:                        Option<u8>,
    inline_function_expansion:           Option<u8>,
    enable_intrinsic_functions:          Option<bool>,
    omit_frame_pointers:                 Option<bool>,
    enable_fiber_safe_optimizations:     Option<bool>,
    // Requires interpolation: $(SolutionDir)/stlport;
    additional_include_directories:      Option<Vec<String>>,
    // PreprocessorDefinitions="WIN32;NDEBUG;VOSTOK_STATIC_LIBRARIES;MASTER_GOLD;"
    preprocessor_definitions:            Option<Vec<String>>,
    string_pooling:                      Option<bool>,
    minimal_rebuild:                     Option<bool>,
    exception_handling:                  Option<u8>,
    runtime_library:                     Option<u8>,
    buffer_security_check:               Option<bool>,
    enable_enhanced_instruction_set:     Option<u8>,
    floating_point_model:                Option<u8>,
    use_precompiled_header:              Option<u8>,
    precompiled_header_through:          Option<String>,
    warning_level:                       Option<u8>,
    detect_64bit_portability_problems:   Option<bool>,
    debug_information_format:            Option<u8>,
    basic_runtime_checks:                Option<u8>,
    favor_size_or_speed:                 Option<u8>,
    enable_function_level_linking:       Option<bool>,
    precompiled_header_file:             Option<String>,
    whole_program_optimization:          Option<bool>,
    smaller_type_check:                  Option<bool>,
    assembler_listing_location:          Option<String>,
    browse_information:                  Option<u8>,
    calling_convention:                  Option<u8>,
    compile_as:                          Option<u8>,
    // NOTE: can be ';' or ',' separated depending on project
    disable_specific_warnings:           Option<Vec<String>>,
    floating_point_exceptions:           Option<bool>,
    force_conformance_in_for_loop_scope: Option<bool>,
    generate_preprocessed_file:          Option<u8>,
    object_file:                         Option<String>,
    program_data_base_file_name:         Option<String>,
    runtime_type_info:                   Option<bool>,
    show_includes:                       Option<bool>,
    struct_member_alignment:             Option<u8>,
    suppress_startup_banner:             Option<bool>,
    use_unicode_response_files:          Option<bool>,
    execution_bucket:                    Option<u8>,
}

#[derive(Debug)]
#[rustfmt::skip]
pub struct LinkerTool {
    additional_options:                 Option<String>,
    additional_dependencies:            Option<Vec<String>>,
    output_file:                        Option<String>,
    link_incremental:                   Option<u8>,
    additional_library_directories:     Option<Vec<String>>,
    ignore_default_library_names:       Option<Vec<String>>,
    module_definition_file:             Option<String>,
    generate_debug_information:         Option<bool>,
    program_database_file:              Option<String>,
    generate_map_file:                  Option<bool>,
    map_file_name:                      Option<String>,
    map_exports:                        Option<bool>,
    sub_system:                         Option<u8>,
    large_address_aware:                Option<u8>,
    optimize_references:                Option<u8>,
    enable_comdat_folding:              Option<u8>,
    randomized_base_address:            Option<u8>,
    data_execution_prevention:          Option<u8>,
    import_library:                     Option<String>,
    target_machine:                     Option<u8>,
    assembly_debug:                     Option<u8>,
    assembly_link_resource:             Option<String>,
    base_address:                       Option<String>,
    clr_thread_attribute:               Option<u8>,
    delay_load_dlls:                    Option<Vec<String>>,
    embed_managed_resource_file:        Option<String>,
    entry_point_symbol:                 Option<String>,
    fixed_base_address:                 Option<u8>,
    generate_manifest:                  Option<bool>,
    ignore_import_library:              Option<bool>,
    optimize_for_windows98:             Option<u8>,
    support_unload_of_delay_loaded_dll: Option<bool>,
    version:                            Option<String>,
}

#[derive(Debug)]
#[rustfmt::skip]
pub struct LibTool {
    additional_options:             Option<String>,
    // Requires interpolation: $(SolutionDir)../binaries/$(PlatformName)/libraries/vostok_$(ProjectName)-static-gold.lib"
    output_file:                    Option<String>, // TODO: nvidia\nvt\project\squish.vcproj
    additional_library_directories: Option<Vec<String>>,
    ignore_default_library_names:   Option<Vec<String>>,
    suppress_startup_banner:        Option<bool>,
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
    unique_identifier: Option<String>,
    filters: Vec<Filter>,
    files: Vec<File>,
}

#[derive(Debug)]
pub struct File {
    relative_path: String,
    file_type: Option<u8>,
    sub_type: Option<String>,
    file_configurations: Vec<FileConfiguration>,
    files: Vec<File>,
}

#[derive(Debug)]
pub struct FileConfiguration {
    name: String,
    excluded_from_build: bool,
    tool: Tool,
}

#[derive(Debug)]
#[rustfmt::skip]
pub struct Tool {
    name:                        String,
    use_precompiled_header:      Option<u8>,
    additional_options:          Option<String>,
    basic_runtime_checks:        Option<u8>,
    disable_specific_warnings:   Option<Vec<String>>,
    generate_preprocessed_file:  Option<u8>,
    object_file:                 Option<String>,
    optimization:                Option<u8>,
    precompiled_header_file:     Option<String>,
    precompiled_header_through:  Option<String>,
    preprocessor_definitions:    Option<Vec<String>>,
    show_includes:               Option<bool>,
    warning_level:               Option<u8>,
    xml_documentation_file_name: Option<String>,
}

macro_rules! parse_attrs {
    ($node:expr, $ctx:literal, {
        $($attr_name:literal => $field:ident,)*
        $(ignore: $ignore:literal,)*
        $(optional: $attr_name_opt:literal => $field_opt:ident,)*
    }) => {
        $(let mut $field: Option<&str> = None;)*
        $(let mut $field_opt: Option<&str> = None;)*

        for attr in $node.attributes() {
            match attr.name() {
                $($ignore)|*|"" => {}
                $($attr_name => _ = $field.replace(attr.value()),)*
                $($attr_name_opt => _ = $field_opt.replace(attr.value()),)*
                attr_name => {
                    anyhow::bail!("Unexpected {} attribute: '{attr_name}' with value: '{}'", $ctx, attr.value())
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
                      "ProjectType"            => project_type,
                      "Version"                => version,
                      "Name"                   => name,
                      "ProjectGUID"            => guid,
                      "RootNamespace"          => root_namespace,
                      "TargetFrameworkVersion" => target_framework_version,
            optional: "Keyword"                => keyword,
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
                    // TODO: 'ode\sources\ode.vcproj'
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
                      "Name"                                  => name,
                      "ConfigurationType"                     => configuraiton_type,
            optional: "WholeProgramOptimization"              => whole_program_optimization,
            optional: "ManagedExtensions"                     => managed_extensions,
            optional: "CharacterSet"                          => character_set,
            optional: "OutputDirectory"                       => output_directory,
            optional: "IntermediateDirectory"                 => intermediate_directory,
            optional: "ATLMinimizesCRunTimeLibraryUsage"      => atl_minimizes_c_runtime_library_usage,
            optional: "DeleteExtensionsOnClean"               => delete_extensions_on_clean,
            optional: "InheritedPropertySheets"               => inherited_property_sheets,
            optional: "UseOfATL"                              => use_of_atl,
            optional: "UseOfMFC"                              => use_of_mfc,
        });

        let name                                   = name.to_string();
        let configuraiton_type                     = configuraiton_type.parse::<u8>()?;
        let whole_program_optimization             = parse_bool(whole_program_optimization.unwrap_or("false"))?;
        optparse!(managed_extensions:                     u8);
        optparse!(character_set:                          u8);
        optparse!(output_directory:                       String);
        optparse!(intermediate_directory:                 String);
        optparse!(atl_minimizes_c_runtime_library_usage:  bool);
        optparse!(delete_extensions_on_clean:             String);
        optparse!(inherited_property_sheets:              Vec<_>);
        optparse!(use_of_atl:                             u8);
        optparse!(use_of_mfc:                             i8);

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
            atl_minimizes_c_runtime_library_usage,
            delete_extensions_on_clean,
            inherited_property_sheets,
            use_of_atl,
            use_of_mfc,

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
            ignore:   "Name",
            optional: "AdditionalOptions"                => additional_options,
            optional: "Optimization"                     => optimization,
            optional: "InlineFunctionExpansion"          => inline_function_expansion,
            optional: "EnableIntrinsicFunctions"         => enable_intrinsic_functions,
            optional: "OmitFramePointers"                => omit_frame_pointers,
            optional: "EnableFiberSafeOptimizations"     => enable_fiber_safe_optimizations,
            optional: "AdditionalIncludeDirectories"     => additional_include_directories,
            optional: "PreprocessorDefinitions"          => preprocessor_definitions,
            optional: "StringPooling"                    => string_pooling,
            optional: "MinimalRebuild"                   => minimal_rebuild,
            optional: "ExceptionHandling"                => exception_handling,
            optional: "RuntimeLibrary"                   => runtime_library,
            optional: "BufferSecurityCheck"              => buffer_security_check,
            optional: "EnableEnhancedInstructionSet"     => enable_enhanced_instruction_set,
            optional: "FloatingPointModel"               => floating_point_model,
            optional: "UsePrecompiledHeader"             => use_precompiled_header,
            optional: "PrecompiledHeaderThrough"         => precompiled_header_through,
            optional: "WarningLevel"                     => warning_level,
            optional: "Detect64BitPortabilityProblems"   => detect_64bit_portability_problems,
            optional: "DebugInformationFormat"           => debug_information_format,
            optional: "BasicRuntimeChecks"               => basic_runtime_checks,
            optional: "FavorSizeOrSpeed"                 => favor_size_or_speed,
            optional: "EnableFunctionLevelLinking"       => enable_function_level_linking,
            optional: "PrecompiledHeaderFile"            => precompiled_header_file,
            optional: "WholeProgramOptimization"         => whole_program_optimization,
            optional: "SmallerTypeCheck"                 => smaller_type_check,
            optional: "AssemblerListingLocation"         => assembler_listing_location,
            optional: "BrowseInformation"                => browse_information,
            optional: "CallingConvention"                => calling_convention,
            optional: "CompileAs"                        => compile_as,
            optional: "DisableSpecificWarnings"          => disable_specific_warnings,
            optional: "FloatingPointExceptions"          => floating_point_exceptions,
            optional: "ForceConformanceInForLoopScope"   => force_conformance_in_for_loop_scope,
            optional: "GeneratePreprocessedFile"         => generate_preprocessed_file,
            optional: "ObjectFile"                       => object_file,
            optional: "ProgramDataBaseFileName"          => program_data_base_file_name,
            optional: "RuntimeTypeInfo"                  => runtime_type_info,
            optional: "ShowIncludes"                     => show_includes,
            optional: "StructMemberAlignment"            => struct_member_alignment,
            optional: "SuppressStartupBanner"            => suppress_startup_banner,
            optional: "UseUnicodeResponseFiles"          => use_unicode_response_files,
            optional: "ExecutionBucket"                  => execution_bucket,
        });

        optparse!(additional_options:                  String);
        optparse!(optimization:                        u8);
        optparse!(inline_function_expansion:           u8);
        optparse!(enable_intrinsic_functions:          bool);
        optparse!(omit_frame_pointers:                 bool);
        optparse!(enable_fiber_safe_optimizations:     bool);
        optparse!(additional_include_directories:      Vec<_>);
        optparse!(preprocessor_definitions:            Vec<_>);
        optparse!(string_pooling:                      bool);
        optparse!(minimal_rebuild:                     bool);
        optparse!(exception_handling:                  u8);
        optparse!(runtime_library:                     u8);
        optparse!(buffer_security_check:               bool);
        optparse!(enable_enhanced_instruction_set:     u8);
        optparse!(floating_point_model:                u8);
        optparse!(use_precompiled_header:              u8);
        optparse!(precompiled_header_through:          String);
        optparse!(warning_level:                       u8);
        optparse!(detect_64bit_portability_problems:   bool);
        optparse!(debug_information_format:            u8);
        optparse!(basic_runtime_checks:                u8);
        optparse!(favor_size_or_speed:                 u8);
        optparse!(enable_function_level_linking:       bool);
        optparse!(precompiled_header_file:             String);
        optparse!(whole_program_optimization:          bool);
        optparse!(smaller_type_check:                  bool);
        optparse!(assembler_listing_location:          String);
        optparse!(browse_information:                  u8);
        optparse!(calling_convention:                  u8);
        optparse!(compile_as:                          u8);
        optparse!(disable_specific_warnings:           Vec<_>);
        optparse!(floating_point_exceptions:           bool);
        optparse!(force_conformance_in_for_loop_scope: bool);
        optparse!(generate_preprocessed_file:          u8);
        optparse!(object_file:                         String);
        optparse!(program_data_base_file_name:         String);
        optparse!(runtime_type_info:                   bool);
        optparse!(show_includes:                       bool);
        optparse!(struct_member_alignment:             u8);
        optparse!(suppress_startup_banner:             bool);
        optparse!(use_unicode_response_files:          bool);
        optparse!(execution_bucket:                    u8);

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
            assembler_listing_location,
            browse_information,
            calling_convention,
            compile_as,
            disable_specific_warnings,
            floating_point_exceptions,
            force_conformance_in_for_loop_scope,
            generate_preprocessed_file,
            object_file,
            program_data_base_file_name,
            runtime_type_info,
            show_includes,
            struct_member_alignment,
            suppress_startup_banner,
            use_unicode_response_files,
            execution_bucket,
        })
    }
}

impl LinkerTool {
    #[rustfmt::skip]
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "VCLinkerTool", {
            ignore:   "Name",
            optional: "AdditionalOptions"               => additional_options,
            optional: "AdditionalDependencies"          => additional_dependencies,
            optional: "OutputFile"                      => output_file,
            optional: "LinkIncremental"                 => link_incremental,
            optional: "AdditionalLibraryDirectories"    => additional_library_directories,
            optional: "IgnoreDefaultLibraryNames"       => ignore_default_library_names,
            optional: "ModuleDefinitionFile"            => module_definition_file,
            optional: "GenerateDebugInformation"        => generate_debug_information,
            optional: "ProgramDatabaseFile"             => program_database_file,
            optional: "GenerateMapFile"                 => generate_map_file,
            optional: "MapFileName"                     => map_file_name,
            optional: "MapExports"                      => map_exports,
            optional: "SubSystem"                       => sub_system,
            optional: "LargeAddressAware"               => large_address_aware,
            optional: "OptimizeReferences"              => optimize_references,
            optional: "EnableCOMDATFolding"             => enable_comdat_folding,
            optional: "RandomizedBaseAddress"           => randomized_base_address,
            optional: "DataExecutionPrevention"         => data_execution_prevention,
            optional: "ImportLibrary"                   => import_library,
            optional: "TargetMachine"                   => target_machine,
            optional: "AssemblyDebug"                   => assembly_debug,
            optional: "AssemblyLinkResource"            => assembly_link_resource,
            optional: "BaseAddress"                     => base_address,
            optional: "CLRThreadAttribute"              => clr_thread_attribute,
            optional: "DelayLoadDLLs"                   => delay_load_dlls,
            optional: "EmbedManagedResourceFile"        => embed_managed_resource_file,
            optional: "EntryPointSymbol"                => entry_point_symbol,
            optional: "FixedBaseAddress"                => fixed_base_address,
            optional: "GenerateManifest"                => generate_manifest,
            optional: "IgnoreImportLibrary"             => ignore_import_library,
            optional: "OptimizeForWindows98"            => optimize_for_windows98,
            optional: "SupportUnloadOfDelayLoadedDLL"   => support_unload_of_delay_loaded_dll,
            optional: "Version"                         => version,
        });

        optparse!(additional_options:                 String);
        optparse!(additional_dependencies:            Vec<_>);
        optparse!(output_file:                        String);
        optparse!(link_incremental:                   u8);
        optparse!(additional_library_directories:     Vec<_>);
        optparse!(ignore_default_library_names:       Vec<_>);
        optparse!(module_definition_file:             String);
        optparse!(generate_debug_information:         bool);
        optparse!(program_database_file:              String);
        optparse!(generate_map_file:                  bool);
        optparse!(map_file_name:                      String);
        optparse!(map_exports:                        bool);
        optparse!(sub_system:                         u8);
        optparse!(large_address_aware:                u8);
        optparse!(optimize_references:                u8);
        optparse!(enable_comdat_folding:              u8);
        optparse!(randomized_base_address:            u8);
        optparse!(data_execution_prevention:          u8);
        optparse!(import_library:                     String);
        optparse!(target_machine:                     u8);
        optparse!(assembly_debug:                     u8);
        optparse!(assembly_link_resource:             String);
        optparse!(base_address:                       String);
        optparse!(clr_thread_attribute:               u8);
        optparse!(delay_load_dlls:                    Vec<_>);
        optparse!(embed_managed_resource_file:        String);
        optparse!(entry_point_symbol:                 String);
        optparse!(fixed_base_address:                 u8);
        optparse!(generate_manifest:                  bool);
        optparse!(ignore_import_library:              bool);
        optparse!(optimize_for_windows98:             u8);
        optparse!(support_unload_of_delay_loaded_dll: bool);
        optparse!(version:                            String);

        Ok(Self {
            additional_options,
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
            assembly_debug,
            assembly_link_resource,
            base_address,
            clr_thread_attribute,
            delay_load_dlls,
            embed_managed_resource_file,
            entry_point_symbol,
            fixed_base_address,
            generate_manifest,
            ignore_import_library,
            optimize_for_windows98,
            support_unload_of_delay_loaded_dll,
            version,
        })
    }
}

impl LibTool {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "VCLibrarianTool", {
            ignore:   "Name",
            optional: "AdditionalOptions"            => additional_options,
            optional: "OutputFile"                   => output_file,
            optional: "AdditionalLibraryDirectories" => additional_library_directories,
            optional: "IgnoreDefaultLibraryNames"    => ignore_default_library_names,
            optional: "SuppressStartupBanner"        => suppress_startup_banner,
        });

        optparse!(additional_options:            String);
        optparse!(output_file:                   String);
        optparse!(additional_library_directories: Vec<_>);
        optparse!(ignore_default_library_names:  Vec<_>);
        optparse!(suppress_startup_banner:       bool);

        Ok(Self {
            additional_options,
            output_file,
            additional_library_directories,
            ignore_default_library_names,
            suppress_startup_banner,
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
                      "Name"             => name,
            optional: "Filter"           => filter,
            optional: "UniqueIdentifier" => unique_identifier,
        });

        let name = name.to_string();
        optparse!(filter: Vec<_>);
        optparse!(unique_identifier: String);

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
            filter,
            unique_identifier,
            filters,
            files,
        })
    }
}

impl File {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "File", {
                      "RelativePath" => relative_path,
            optional: "FileType"     => file_type,
            optional: "SubType"      => sub_type,
        });

        let relative_path = relative_path.to_string();
        optparse!(file_type: u8);
        optparse!(sub_type: String);

        let mut file_configurations = vec![];
        let mut files = vec![];

        for child in node.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "FileConfiguration" => {
                    let file_configuration = FileConfiguration::parse_xml(child)?;
                    file_configurations.push(file_configuration)
                }
                "File" => files.push(File::parse_xml(child)?),
                "Tool" => {}
                tag => anyhow::bail!("Unexpected tag in File: '{tag}'"),
            }
        }

        Ok(Self {
            relative_path,
            file_type,
            sub_type,
            file_configurations,
            files,
        })
    }
}

impl FileConfiguration {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        parse_attrs!(node, "FileConfiguration", {
                      "Name"              => name,
            optional: "ExcludedFromBuild" => excluded_from_build,
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
                      "Name"                     => name,
            optional: "UsePrecompiledHeader"     => use_precompiled_header,
            optional: "AdditionalOptions"        => additional_options,
            optional: "BasicRuntimeChecks"       => basic_runtime_checks,
            optional: "DisableSpecificWarnings"  => disable_specific_warnings,
            optional: "GeneratePreprocessedFile" => generate_preprocessed_file,
            optional: "ObjectFile"               => object_file,
            optional: "Optimization"             => optimization,
            optional: "PrecompiledHeaderFile"    => precompiled_header_file,
            optional: "PrecompiledHeaderThrough" => precompiled_header_through,
            optional: "PreprocessorDefinitions"  => preprocessor_definitions,
            optional: "ShowIncludes"             => show_includes,
            optional: "WarningLevel"             => warning_level,
            optional: "XMLDocumentationFileName" => xml_documentation_file_name,
        });

        let name = name.to_string();
        optparse!(use_precompiled_header:     u8);
        optparse!(additional_options:         String);
        optparse!(basic_runtime_checks:       u8);
        optparse!(disable_specific_warnings:  Vec<_>);
        optparse!(generate_preprocessed_file: u8);
        optparse!(object_file:                String);
        optparse!(optimization:               u8);
        optparse!(precompiled_header_file:    String);
        optparse!(precompiled_header_through: String);
        optparse!(preprocessor_definitions:   Vec<_>);
        optparse!(show_includes:              bool);
        optparse!(warning_level:              u8);
        optparse!(xml_documentation_file_name: String);

        Ok(Self {
            name,
            use_precompiled_header,
            additional_options,
            basic_runtime_checks,
            disable_specific_warnings,
            generate_preprocessed_file,
            object_file,
            optimization,
            precompiled_header_file,
            precompiled_header_through,
            preprocessor_definitions,
            show_includes,
            warning_level,
            xml_documentation_file_name,
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
