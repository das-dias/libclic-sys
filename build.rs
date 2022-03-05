fn main()
{
    let src = [
        "src/include/clic/src/libcliccommand.c",
        "src/include/clic/src/libclicshell.c",
    ];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("src/include");
    build.compile("clic");
}