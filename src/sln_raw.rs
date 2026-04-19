use nom::{
    bytes::complete::{tag, take_until, take_until1, take_while},
    character::complete::{char, digit1},
    combinator::{map_res, opt},
    multi::many0,
    sequence::{preceded, terminated},
    Parser,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct Sln {
    pub projects: Vec<Project>,
    pub global: Global,
}

#[derive(Default, Debug)]
pub struct Project {
    pub up_uuid: Uuid,
    pub name: String,
    pub path: String,
    pub uuid: Uuid,

    pub section_dependencies: Option<SectionDependencies>,
}

#[derive(Default, Debug)]
pub struct SectionDependencies {
    pub deps: Vec<SectionDependency>,
}

#[derive(Debug)]
pub struct SectionDependency {
    pub from: Uuid,
    pub to: Uuid,
    // should be equal?
}

//
//

#[derive(Default, Debug)]
pub struct Global {
    pub pre_sln_platforms: Option<SolutionConfigurationPlatforms>,
    pub post_proj_platforms: Option<ProjectConfigurationPlatforms>,
    pub pre_props: Option<SolutionProperties>,
}

#[derive(Debug)]
pub struct SolutionConfigurationPlatforms {
    pub platforms: Vec<SolutionConfigurationPlatform>,
}

#[derive(Debug)]
pub struct SolutionConfigurationPlatform {
    pub platform_name: String,
}

#[derive(Debug)]
pub struct ProjectConfigurationPlatforms {
    pub platforms: Vec<ProjectConfigurationPlatform>,
}

#[derive(Debug)]
pub struct ProjectConfigurationPlatform {
    pub uuid: Uuid,
    pub sln_platform_name: String,
    pub is_enabled: bool,
}

#[derive(Debug)]
pub struct SolutionProperties {
    pub hide_solution_node: bool,
}

#[derive(Debug)]
pub struct NestedProjects {
    pub from: Uuid,
    pub to: Uuid,
}

impl Sln {
    pub fn parse(i: &str) -> nom::IResult<&str, Self> {
        let (i, _) = opt(tag("\u{FEFF}")).parse(i)?;

        let (i, _) = sp(i)?;
        let (i, (major_version, minor_version)) = sln_version(i)?;
        if major_version != 10 && minor_version != 0 {
            panic!("Unknown version: {major_version}.{minor_version}");
        }

        let (i, _) = sp(i)?;
        let (i, vs_version) = vs_version(i)?;
        if vs_version != 2008 {
            panic!("Unknown VS version: {vs_version}");
        }

        let (i, _) = sp(i)?;
        let (i, projects) = many0(Project::parse).parse(i)?;

        let (i, _) = sp(i)?;
        let (i, global) = Global::parse(i)?;

        Ok((i, Self { projects, global }))
    }
}

impl Project {
    // Project("{2150E333-8FDC-42A3-9474-1A3956D46DE8}") = "survarium", "survarium", "{4E2399DA-D511-4F61-ACCB-894F87214FC5}"
    // EndProject
    pub fn parse<'a>(i: &'a str) -> nom::IResult<&'a str, Self> {
        let (i, _) = tag("Project").parse(i)?;

        let (i, up_uuid) = parse_parentheses(i)?;
        let (j, up_uuid) = parse_uuid(up_uuid)?;
        assert_eq!(j, "");

        let (i, _) = charsep('=').parse(i)?;

        let (i, name) = parse_string(i)?;
        let (i, _) = charsep(',').parse(i)?;

        let (i, path) = parse_string(i)?;
        let (i, _) = charsep(',').parse(i)?;

        let (i, uuid) = parse_uuid(i)?;
        let (i, _) = sp(i)?;

        let (i, section_dependencies) = opt(SectionDependencies::parse).parse(i)?;

        let (i, _) = tag("EndProject").parse(i)?;
        let (i, _) = sp(i)?;

        Ok((
            i,
            Self {
                up_uuid,
                name: name.to_string(),
                path: path.to_string(),
                uuid,
                section_dependencies,
            },
        ))
    }
}

// ProjectSection(ProjectDependencies) = postProject
// 	{CE017322-01FC-4851-9C8B-64E9A8E26C38} = {CE017322-01FC-4851-9C8B-64E9A8E26C38}
// 	{F143D180-D4C4-4037-B3DE-BE89A21C8D1D} = {F143D180-D4C4-4037-B3DE-BE89A21C8D1D}
// 	{4046F392-A18B-4C66-9639-3EABFFF5D531} = {4046F392-A18B-4C66-9639-3EABFFF5D531}
// EndProjectSection
impl SectionDependencies {
    pub fn parse<'a>(i: &'a str) -> nom::IResult<&'a str, Self> {
        let (i, _) = tag("ProjectSection(ProjectDependencies) = postProject").parse(i)?;
        let (i, _) = sp(i)?;

        let (i, deps) = many0(SectionDependency::parse).parse(i)?;

        let (i, _) = tag("EndProjectSection").parse(i)?;
        let (i, _) = sp(i)?;

        Ok((i, Self { deps }))
    }
}

// 	{4046F392-A18B-4C66-9639-3EABFFF5D531} = {4046F392-A18B-4C66-9639-3EABFFF5D531}
impl SectionDependency {
    pub fn parse<'a>(i: &'a str) -> nom::IResult<&'a str, Self> {
        let (i, from) = parse_uuid_raw(i)?;
        let (i, _) = charsep('=').parse(i)?;

        let (i, to) = parse_uuid_raw(i)?;
        let (i, _) = sp(i)?;

        Ok((i, Self { from, to }))
    }
}

impl Global {
    pub fn parse<'a>(i: &'a str) -> nom::IResult<&'a str, Self> {
        Ok((i, Self::default()))
    }
}

fn parse_uuid(i: &str) -> nom::IResult<&str, Uuid> {
    let (i, up_uuid) = parse_string(i)?;
    let (j, up_uuid) = parse_uuid_raw(up_uuid)?;
    assert_eq!(j, "");

    Ok((i, up_uuid))
}

fn parse_uuid_raw(i: &str) -> nom::IResult<&str, Uuid> {
    let (i, up_uuid) = parse_curly(i)?;

    let up_uuid = Uuid::parse_str(up_uuid)
        .map_err(|_| nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Fail)))?;

    Ok((i, up_uuid))
}

fn parse_curly(i: &str) -> nom::IResult<&str, &str> {
    preceded(char('{'), terminated(take_until("}"), char('}'))).parse(i)
}

fn parse_parentheses(i: &str) -> nom::IResult<&str, &str> {
    preceded(char('('), terminated(take_until(")"), char(')'))).parse(i)
}

fn parse_string(i: &str) -> nom::IResult<&str, &str> {
    preceded(char('"'), terminated(take_until("\""), char('"'))).parse(i)
}

fn vs_version(i: &str) -> nom::IResult<&str, u16> {
    let (i, _) = tag("# Visual Studio ").parse(i)?;
    let (i, version) = map_res(digit1, |res| str::parse::<u16>(res)).parse(i)?;

    Ok((i, version))
}

fn sln_version(i: &str) -> nom::IResult<&str, (u8, u8)> {
    let (i, _) = tag("Microsoft Visual Studio Solution File, Format Version ").parse(i)?;
    let (i, major_version) = map_res(digit1, |res| str::parse::<u8>(res)).parse(i)?;
    let (i, _) = tag(".").parse(i)?;
    let (i, minor_version) = map_res(digit1, |res| str::parse::<u8>(res)).parse(i)?;

    Ok((i, (major_version, minor_version)))
}

fn charsep(sep: char) -> impl FnMut(&str) -> nom::IResult<&str, char> {
    move |i: &str| {
        let (i, _) = sp(i)?;
        let (i, sep) = char(sep).parse(i)?;
        let (i, _) = sp(i)?;

        Ok((i, sep))
    }
}

fn sp(i: &str) -> nom::IResult<&str, &str> {
    let chars = " \t\r\n";

    take_while(move |c| chars.contains(c)).parse(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_sln_version() {
        let input = "Microsoft Visual Studio Solution File, Format Version 12.13";
        assert_eq!(sln_version(input).unwrap().1, (12, 13));
    }

    #[test]
    fn parses_escaped_str() {
        let input = "\"hello\"";
        assert_eq!(parse_string(input).unwrap().1, "hello");
    }

    #[test]
    fn parses_empty_project() {
        let input = r#"
Project("{2150E333-8FDC-42A3-9474-1A3956D46DE8}") = "survarium", "survarium", "{4E2399DA-D511-4F61-ACCB-894F87214FC5}"
EndProject
"#.trim();

        let project = Project::parse(input).unwrap().1;
        assert_eq!(project.name, "survarium");
        assert_eq!(project.path, "survarium");
        assert_eq!(
            project.up_uuid,
            Uuid::parse_str("{2150E333-8FDC-42A3-9474-1A3956D46DE8}").unwrap()
        );
        assert_eq!(
            project.uuid,
            Uuid::parse_str("{4E2399DA-D511-4F61-ACCB-894F87214FC5}").unwrap()
        );
    }

    #[test]
    fn parse_project() {
        let input = r#"
Project("{8BC9CEB8-8B4A-11D0-8D11-00A0C91BC942}") = "bugtrap", "BugTrap\BugTrap.vcproj", "{E8CF1ADA-264A-4127-86C2-FD7057D3792C}"
	ProjectSection(ProjectDependencies) = postProject
		{CA2604CA-A2AC-49A1-A468-8AB5A2E6CBC9} = {CA2604CA-A2AC-49A1-A468-8AB5A2E6CBC9}
		{893279CB-0805-405F-B484-9BB728A18261} = {893279CB-0805-405F-B484-9BB728A18261}
	EndProjectSection
EndProject
"#.trim();

        let project = Project::parse(input).unwrap().1;
        assert_eq!(project.name, "bugtrap");
        assert_eq!(project.path, "BugTrap\\BugTrap.vcproj");
        assert_eq!(project.section_dependencies.unwrap().deps.len(), 2);
    }

    #[test]
    fn hello() {
        let mut parser = preceded(charsep('('), tag("hello"));

        assert_eq!(parser.parse("   (    hello").unwrap().1, "hello");
        assert_eq!(parser.parse("(hello").unwrap().1, "hello");
        assert_eq!(parser.parse("\n\n(         \r\thello").unwrap().1, "hello");
    }
}
