1. Claim section:
    All Tasks (T1, T2, T3, T4, T5) were implemented.
2. Tested environments:
    OS: Windows 10
3. Additional and general remarks:
    The recommended literature `https://raytracing.github.io/books/RayTracingInOneWeekend.html`was used.
    Regarding the deserialization of XML-files: the following instructions were followed `https://docs.rs/quick-xml/latest/quick_xml/`
    I used the chromaticities given on the website to ensure equal colors `https://docs.rs/png/0.17.9/png/`
    In order to start the program you need execute the following command in the root folder of the project
    * `cargo run --release <path to XML-File>`
    Example command (PowerShell)
    * `cargo run --release .\scenes\example2.xml`
