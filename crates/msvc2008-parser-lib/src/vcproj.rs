use anyhow::Context;
use msvc2008_parser_proc::{flag_enum, ParseXml};

#[derive(Debug, ParseXml)]
pub struct VCProject {
    pub name: String,
    pub project_type: String,
    pub version: String,
    #[rename("ProjectGUID")]
    pub guid: String,
    pub root_namespace: String,
    pub keyword: Option<String>, // TODO: freeimage\LibOpenJPEG\LibOpenJPEG.vcproj
    pub target_framework_version: String,

    #[skip]
    pub platforms: Vec<Platform>,
    #[skip]
    pub configurations: Vec<Configuration>,
    #[skip]
    pub files: Files,
}

#[derive(Debug, ParseXml)]
pub struct Configuration {
    pub name: String,
    // Requires interpolation:
    // OutputDirectory="$(SolutionDir)../binaries/$(PlatformName)/intermediates/$(ConfigurationName)/$(ProjectName)"
    pub output_directory: Option<String>, // TODO: Can be missing in game in random config
    // Requires interpolation:
    // IntermediateDirectory="$(SolutionDir)../binaries/$(PlatformName)/intermediates/$(ConfigurationName)/$(ProjectName)"
    pub intermediate_directory: Option<String>, // TODO: Same as above
    pub configuration_type: u8,                 // ??
    pub character_set: Option<u8>,
    pub whole_program_optimization: Option<bool>,
    pub managed_extensions: Option<u8>,
    #[rename("ATLMinimizesCRunTimeLibraryUsage")]
    pub atl_minimizes_c_runtime_library_usage: Option<bool>,
    pub delete_extensions_on_clean: Option<String>,
    pub inherited_property_sheets: Option<Vec<String>>,
    #[rename("UseOfATL")]
    pub use_of_atl: Option<u8>,
    #[rename("UseOfMFC")]
    pub use_of_mfc: Option<i8>,

    #[skip]
    pub compiler_tool: Option<CompilerTool>, // TODO: Can be missing for Xbox
    #[skip]
    pub lib_tool: Option<LibTool>, // Should be either lib or linker
    #[skip]
    pub linker_tool: Option<LinkerTool>,
}

#[derive(Debug, ParseXml)]
pub struct Platform {
    pub name: String,
}

#[derive(Debug, ParseXml)]
#[parse_xml(tag = "VCCLCompilerTool", ignore = "Name")]
pub struct CompilerTool {
    pub additional_options: Option<String>,
    pub optimization: Option<u8>,
    pub inline_function_expansion: Option<u8>,
    pub enable_intrinsic_functions: Option<bool>,
    pub omit_frame_pointers: Option<bool>,
    pub enable_fiber_safe_optimizations: Option<bool>,
    // Requires interpolation: $(SolutionDir)/stlport;
    pub additional_include_directories: Option<Vec<String>>,
    // PreprocessorDefinitions="WIN32;NDEBUG;VOSTOK_STATIC_LIBRARIES;MASTER_GOLD;"
    pub preprocessor_definitions: Option<Vec<String>>,
    pub string_pooling: Option<bool>,
    pub minimal_rebuild: Option<bool>,
    pub exception_handling: Option<u8>,
    pub runtime_library: Option<u8>,
    pub buffer_security_check: Option<bool>,
    pub enable_enhanced_instruction_set: Option<u8>,
    pub floating_point_model: Option<u8>,
    pub use_precompiled_header: Option<u8>,
    pub precompiled_header_through: Option<String>,
    pub warning_level: Option<u8>,
    pub detect_64_bit_portability_problems: Option<bool>,
    pub debug_information_format: Option<u8>,
    pub basic_runtime_checks: Option<u8>,
    pub favor_size_or_speed: Option<u8>,
    pub enable_function_level_linking: Option<bool>,
    pub precompiled_header_file: Option<String>,
    pub whole_program_optimization: Option<bool>,
    pub smaller_type_check: Option<bool>,
    pub assembler_listing_location: Option<String>,
    pub browse_information: Option<u8>,
    pub calling_convention: Option<u8>,
    pub compile_as: Option<u8>,
    // NOTE: can be ';' or ',' separated depending on project
    // TODO: Forgot how I wrote this, XD
    // TODO: We might not want to split this at all
    pub disable_specific_warnings: Option<Vec<String>>,
    pub floating_point_exceptions: Option<bool>,
    pub force_conformance_in_for_loop_scope: Option<bool>,
    pub generate_preprocessed_file: Option<u8>,
    pub object_file: Option<String>,
    pub program_data_base_file_name: Option<String>,
    pub runtime_type_info: Option<bool>,
    pub show_includes: Option<bool>,
    pub struct_member_alignment: Option<u8>,
    pub suppress_startup_banner: Option<bool>,
    pub use_unicode_response_files: Option<bool>,
    pub execution_bucket: Option<u8>,
}

#[derive(Debug, ParseXml)]
#[parse_xml(tag = "VCLinkerTool", ignore = "Name")]
pub struct LinkerTool {
    pub additional_options: Option<String>,
    pub additional_dependencies: Option<Vec<String>>,
    pub output_file: Option<String>,
    pub link_incremental: Option<u8>,
    pub additional_library_directories: Option<Vec<String>>,
    pub ignore_default_library_names: Option<Vec<String>>,
    pub module_definition_file: Option<String>,
    pub generate_debug_information: Option<bool>,
    pub program_database_file: Option<String>,
    pub generate_map_file: Option<bool>,
    pub map_file_name: Option<String>,
    pub map_exports: Option<bool>,
    pub sub_system: Option<u8>,
    pub large_address_aware: Option<u8>,
    pub optimize_references: Option<u8>,
    #[rename("EnableCOMDATFolding")]
    pub enable_comdat_folding: Option<u8>,
    pub randomized_base_address: Option<u8>,
    pub data_execution_prevention: Option<u8>,
    pub import_library: Option<String>,
    pub target_machine: Option<u8>,
    pub assembly_debug: Option<u8>,
    pub assembly_link_resource: Option<String>,
    pub base_address: Option<String>,
    #[rename("CLRThreadAttribute")]
    pub clr_thread_attribute: Option<u8>,
    #[rename("DelayLoadDLLs")]
    pub delay_load_dlls: Option<Vec<String>>,
    pub embed_managed_resource_file: Option<String>,
    pub entry_point_symbol: Option<String>,
    pub fixed_base_address: Option<u8>,
    pub generate_manifest: Option<bool>,
    pub ignore_import_library: Option<bool>,
    pub optimize_for_windows98: Option<u8>,
    #[rename("SupportUnloadOfDelayLoadedDLL")]
    pub support_unload_of_delay_loaded_dll: Option<bool>,
    pub version: Option<String>,
}

#[derive(Debug, ParseXml)]
#[parse_xml(tag = "VCLibrarianTool", ignore = "Name")]
pub struct LibTool {
    pub additional_options: Option<String>,
    // Requires interpolation: $(SolutionDir)../binaries/$(PlatformName)/libraries/vostok_$(ProjectName)-static-gold.lib"
    pub output_file: Option<String>, // TODO: nvidia\nvt\project\squish.vcproj
    pub additional_library_directories: Option<Vec<String>>,
    pub ignore_default_library_names: Option<Vec<String>>,
    pub suppress_startup_banner: Option<bool>,
}

#[derive(Debug, Default)]
pub struct Files {
    pub filters: Vec<Filter>,
    pub files: Vec<File>,
}

#[derive(Debug, ParseXml)]
pub struct Filter {
    pub name: String,
    pub filter: Option<Vec<String>>,
    pub unique_identifier: Option<String>,

    #[skip]
    pub filters: Vec<Filter>,
    #[skip]
    pub files: Vec<File>,
}

#[derive(Debug, ParseXml)]
pub struct File {
    pub relative_path: String,
    pub file_type: Option<u8>,
    pub sub_type: Option<String>,

    #[skip]
    pub file_configurations: Vec<FileConfiguration>,
    #[skip]
    pub files: Vec<File>,
}

#[derive(Debug, ParseXml)]
pub struct FileConfiguration {
    pub name: String,
    pub excluded_from_build: Option<bool>,

    #[skip]
    pub tool: Tool,
}

#[derive(Debug, Default, ParseXml)]
pub struct Tool {
    pub name: String,
    pub use_precompiled_header: Option<u8>,
    pub additional_options: Option<String>,
    pub basic_runtime_checks: Option<u8>,
    pub disable_specific_warnings: Option<Vec<String>>,
    pub generate_preprocessed_file: Option<u8>,
    pub object_file: Option<String>,
    pub optimization: Option<u8>,
    pub precompiled_header_file: Option<String>,
    pub precompiled_header_through: Option<String>,
    pub preprocessor_definitions: Option<Vec<String>>,
    pub show_includes: Option<bool>,
    pub warning_level: Option<u8>,
    #[rename("XMLDocumentationFileName")]
    pub xml_documentation_file_name: Option<String>,
}

flag_enum! {
    enum Optimization {
        0 => "/Od",
        1 => "/O1",
        2 => "/O2",
        3 => "/Ox",
    }
}

//
// Custom parser logic
//

impl VCProject {
    pub fn parse_xml(input: &str) -> anyhow::Result<Self> {
        let xml = roxmltree::Document::parse(input)?;
        let root = xml.root_element();
        if root.tag_name().name() != "VisualStudioProject" {
            anyhow::bail!("Expected 'VisualStudioProject' as a root object")
        }

        let mut this = Self::parse_xml_inner(root)?;

        for child in root.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "Platforms" => {
                    for node in child
                        .children()
                        .filter(|n| n.is_element() && n.tag_name().name() == "Platform")
                    {
                        this.platforms.push(Platform::parse_xml(node)?);
                    }
                }
                "Configurations" => {
                    for node in child
                        .children()
                        .filter(|n| n.is_element() && n.tag_name().name() == "Configuration")
                    {
                        this.configurations.push(Configuration::parse_xml(node)?);
                    }
                }
                "Files" => {
                    this.files = Files::parse_xml(child)?;
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

        Ok(this)
    }
}

impl Configuration {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        let mut this = Self::parse_xml_inner(node)?;

        for child in node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "Tool")
        {
            match child.attribute("Name") {
                // TODO
                Some("VCCLCompilerTool") => {
                    this.compiler_tool = Some(CompilerTool::parse_xml(child)?)
                }
                Some("VCLibrarianTool") => this.lib_tool = Some(LibTool::parse_xml(child)?),
                Some("VCLinkerTool") => this.linker_tool = Some(LinkerTool::parse_xml(child)?),
                _ => (),
            }
        }

        Ok(this)
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
        let mut this = Self::parse_xml_inner(node)?;

        for child in node.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "Filter" => this.filters.push(Filter::parse_xml(child)?),
                "File" => this.files.push(File::parse_xml(child)?),
                tag => anyhow::bail!("Unexpected tag in Filter: '{tag}'"),
            }
        }

        Ok(this)
    }
}

impl File {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        let mut this = Self::parse_xml_inner(node)?;

        for child in node.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "FileConfiguration" => {
                    let file_configuration = FileConfiguration::parse_xml(child)?;
                    this.file_configurations.push(file_configuration)
                }
                "File" => this.files.push(File::parse_xml(child)?),
                "Tool" => {}
                tag => anyhow::bail!("Unexpected tag in File: '{tag}'"),
            }
        }

        Ok(this)
    }
}

impl FileConfiguration {
    pub fn parse_xml(node: roxmltree::Node) -> anyhow::Result<Self> {
        let mut this = Self::parse_xml_inner(node)?;

        this.tool = node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "Tool")
            .next()
            .context("FileConfiguration missing Tool")
            .and_then(Tool::parse_xml)?;

        Ok(this)
    }
}

//
// macro_rules! used by proc-macro
//

#[rustfmt::skip]
macro_rules! optparse {
    ($f:ident: bool)   => { let $f = $f.map(parse_bool).transpose()?; };
    ($f:ident: String) => { let $f = $f.map(str::to_string); };
    ($f:ident: Vec<_>) => { let $f = $f.map(parse_list); };
    ($f:ident: $t:ty)  => { let $f = $f.map(|s| s.parse::<$t>()).transpose()?; };
}
pub(crate) use optparse;

#[rustfmt::skip]
macro_rules! parse {
    ($f:ident: bool)   => { let $f = parse_bool($f)?; };
    ($f:ident: String) => { let $f = $f.to_string(); };
    ($f:ident: Vec<_>) => { let $f = parse_list($f); };
    ($f:ident: $t:ty)  => { let $f = $f.parse::<$t>()?; };
}
pub(crate) use parse;

macro_rules! parse_attrs {
    ($node:expr, $ctx:literal, {
        $($attr_name:literal => $field:ident,)*
        $(optional: $attr_name_opt:literal => $field_opt:ident,)*
        $(ignore: $ignore:literal,)*
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
pub(crate) use parse_attrs;

//
// Helpers used by macro_rules!
//

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
