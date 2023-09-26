1. Claim section:
    All tasks (T1, T2, T3, T4, T5, T6, T7) were implemented.
    Implemented Effects: EF3 (anti), EF8 (spot)
    Implemented Bonus: B6 (supersampling)
2. Tested environments:
    OS: Windows 10
3. Additional and general remarks:
    The recommended literature `https://raytracing.github.io/books/RayTracingInOneWeekend.html`was used.
    Regarding the deserialization of XML-files: the following instructions were followed `https://docs.rs/quick-xml/latest/quick_xml/`
    I used the chromaticities given on the website to ensure equal colors `https://docs.rs/png/0.17.9/png/`
    The given code in the section `Using the decoder` has been used to implement loading texture files https://docs.rs/png/latest/png/
    In order to start the program you need execute the following command in the root folder of the project
    * `cargo run --release <path to XML-File>`
    Example command (PowerShell)
    * `cargo run --release .\scenes\example2.xml`

    In order to enable supersampling, the flag `-s=<USIZE_NUMBER>` is required
    Example command (PowerShell)
    * `cargo run --release .\scenes\example4.xml -s=128`
    Anti-aliased textures are enabled per default
    For spotlight, please consider rendering `spotlight.xml`
    
