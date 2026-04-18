use nom::{
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    combinator::{map_res, opt},
    error::{ContextError, ParseError},
    multi::{many, many0},
    number::complete::u8 as parse_u8,
    IResult, Parser,
};
use sha1::Sha1;

pub struct Sln {
    pub projects: Vec<Project>,
    pub global: Global,
}

#[derive(Default)]
pub struct Project {
    pub up_hash: Sha1,
    pub name: String,
    pub path: String,
    pub hash: Sha1,

    pub section_dependencies: Option<SectionDependencies>,
}

#[derive(Default)]
pub struct SectionDependencies {
    pub deps: Vec<SectionDependency>,
}

pub struct SectionDependency {
    pub hash1: Sha1,
    pub hash2: Sha1,
    // should be equal?
}

//
//

#[derive(Default)]
pub struct Global {
    pub pre_sln_platforms: Option<SolutionConfigurationPlatforms>,
    pub post_proj_platforms: Option<ProjectConfigurationPlatforms>,
    pub pre_props: Option<SolutionProperties>,
}

pub struct SolutionConfigurationPlatforms {
    pub platforms: Vec<SolutionConfigurationPlatform>,
}

pub struct SolutionConfigurationPlatform {
    pub platform_name: String,
}

pub struct ProjectConfigurationPlatforms {
    pub platforms: Vec<ProjectConfigurationPlatform>,
}

pub struct ProjectConfigurationPlatform {
    pub hash: Sha1,
    pub sln_platform_name: String,
    pub is_enabled: bool,
}

pub struct SolutionProperties {
    pub hide_solution_node: bool,
}

pub struct NestedProjects {
    pub from: Sha1,
    pub to: Sha1,
}

impl Sln {
    pub fn parse<'a>(i: &'a str) -> Result<Self, nom::Err<nom::error::Error<&'a str>>> {
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
        let (_, global) = Global::parse(i)?;

        Ok(Self { projects, global })
    }
}

impl Project {
    pub fn parse<'a>(i: &'a str) -> nom::IResult<&'a str, Self> {
        let (i, _) = tag("Project").parse(i)?;
        // ..
        let (i, _) = tag("EndProject").parse(i)?;

        Ok((i, Self::default()))
    }
}
impl Global {
    pub fn parse<'a>(i: &'a str) -> nom::IResult<&'a str, Self> {
        Ok((i, Self::default()))
    }
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
}
