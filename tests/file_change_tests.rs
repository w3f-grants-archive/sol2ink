use convert_case::{
    Case::Snake,
    Casing,
};
use std::{
    fs,
    process::Command,
};

macro_rules! implementation_file {
    ($origin:expr,$contract_folder:expr,$contract_name:expr,$file_name:expr) => {
        fs::read_to_string(format!(
            "{}/{}/{}/{}/{}/{}",
            $origin,
            $contract_folder,
            $contract_name,
            $contract_name,
            $contract_name.to_case(Snake),
            $file_name
        ))
        .unwrap_or_else(|err| panic!("{err:?}"))
    };
}

macro_rules! contract_file {
    ($origin:expr, $contract_folder:expr, $contract_name:expr, $file_name:expr) => {
        fs::read_to_string(format!(
            "{}/{}/{}/{}/contract/{}",
            $origin, $contract_folder, $contract_name, $contract_name, $file_name
        ))
        .unwrap_or_else(|err| panic!("{err:?}"))
    };
}

macro_rules! test_case_contract {
    ($contract_folder:expr,$contract_name:expr) => {
        let contract_cargo =
            contract_file!("tests", $contract_folder, $contract_name, "Cargo.toml");
        let contract_lib = contract_file!("tests", $contract_folder, $contract_name, "lib.rs");

        let impl_cargo =
            implementation_file!("tests", $contract_folder, $contract_name, "Cargo.toml");
        let impl_impl = implementation_file!("tests", $contract_folder, $contract_name, "impls.rs");
        let impl_lib = implementation_file!("tests", $contract_folder, $contract_name, "lib.rs");
        let impl_traits =
            implementation_file!("tests", $contract_folder, $contract_name, "traits.rs");

        Command::new("cargo")
            .args([
                "+nightly",
                "run",
                format!(
                    "examples/{}/{}/{}.sol",
                    $contract_folder, $contract_name, $contract_name
                )
                .as_str(),
            ])
            .output()
            .expect("failed to execute process");

        assert_eq!(
            contract_cargo,
            contract_file!("examples", $contract_folder, $contract_name, "Cargo.toml")
        );
        assert_eq!(
            contract_lib,
            contract_file!("examples", $contract_folder, $contract_name, "lib.rs")
        );

        assert_eq!(
            impl_cargo,
            implementation_file!("examples", $contract_folder, $contract_name, "Cargo.toml")
        );
        assert_eq!(
            impl_impl,
            implementation_file!("examples", $contract_folder, $contract_name, "impls.rs")
        );
        assert_eq!(
            impl_lib,
            implementation_file!("examples", $contract_folder, $contract_name, "lib.rs")
        );
        assert_eq!(
            impl_traits,
            implementation_file!("examples", $contract_folder, $contract_name, "traits.rs")
        );
    };
}

macro_rules! test_case_file {
    ($dest:expr,$file_name:expr) => {
        let file = fs::read_to_string(format!("tests/{}/{}.rs", $dest, $file_name)).unwrap();

        Command::new("cargo")
            .args([
                "+nightly",
                "run",
                format!("examples/{}/{}/{}.sol", $dest, $file_name, $file_name).as_str(),
            ])
            .output()
            .expect("failed to execute process");

        assert_eq!(
            file,
            fs::read_to_string(format!(
                "examples/{}/{}/{}.rs",
                $dest, $file_name, $file_name
            ))
            .unwrap()
        );
    };
}

#[test]
fn erc20_is_not_changed() {
    test_case_contract!("contracts", "ERC20");
}

#[test]
fn erc721_is_not_changed() {
    test_case_contract!("contracts", "ERC721");
}

#[test]
fn erc1155_is_not_changed() {
    test_case_contract!("contracts", "ERC1155");
}

#[test]
fn access_control_is_not_changed() {
    test_case_contract!("contracts", "AccessControl");
}

#[test]
fn solang_example_is_not_changed() {
    test_case_contract!("contracts", "example");
}

#[test]
fn flipper_is_not_changed() {
    test_case_contract!("contracts", "flipper");
}

#[test]
fn primitives_is_not_changed() {
    test_case_contract!("contracts", "Primitives");
}

#[test]
fn ierc20_is_not_changed() {
    test_case_file!("interfaces", "IERC20");
}

#[test]
fn ierc721_is_not_changed() {
    test_case_file!("interfaces", "IERC721");
}

#[test]
fn ierc1155_is_not_changed() {
    test_case_file!("interfaces", "IERC1155");
}

#[test]
fn iaccess_control_is_not_changed() {
    test_case_file!("interfaces", "IAccessControl");
}

#[test]
fn safe_math_is_not_changed() {
    test_case_file!("libraries", "SafeMath");
}

#[test]
fn array_contract_is_not_changed() {
    test_case_contract!("test_contracts", "ArrayContract");
}

#[test]
fn comment_contract_is_not_changed() {
    test_case_contract!("test_contracts", "CommentContract");
}

#[test]
fn function_contract_is_not_changed() {
    test_case_contract!("test_contracts", "FunctionContract");
}

#[test]
fn struct_contract_is_not_changed() {
    test_case_contract!("test_contracts", "StructContract");
}
