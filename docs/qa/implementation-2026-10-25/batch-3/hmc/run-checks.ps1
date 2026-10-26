$ErrorActionPreference = 'Stop'
$deps = 'C:/github/ecopunk/target/debug/deps'
$task = $PSScriptRoot
$rustArgs = @('--test','--edition=2021',"$task/harness/lib.rs",
 '--extern',"qualia_core_db=$deps/libqualia_core_db.rlib",
 '--extern',"serde=$deps/libserde-062e9053eb49c468.rlib",
 '--extern',"serde_json=$deps/libserde_json-2562565fe7e871bd.rlib",
 '--extern',"vibe=$deps/libvibe-09a4340642d70c17.rlib",
 '--extern',"tempfile=$deps/libtempfile-1f6fafc1cc17d976.rlib",
 '--extern',"sha2=$deps/libsha2-79f0c5338b48e64b.rlib",
 '-L',"dependency=$deps",
 '-L','native=C:/github/ecopunk/target/debug/build/blake3-da8ba380479449f1/out',
 '-L','native=C:/github/ecopunk/target/debug/build/ring-b53baf53c685dea7/out',
 '-L','native=C:/github/ecopunk/target/debug/build/zstd-sys-ddb15c1df437b75d/out',
 '-L','native=C:/Users/tim_h/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/winapi-x86_64-pc-windows-gnu-0.4.0/lib',
 '-L','native=C:/github/qualiaDB/vendor/directml/bin/x64-win',
 '-o',"$task/harness/checkpoint-tests.exe")
& rustc @rustArgs *> "$task/compile.log"
if ($LASTEXITCODE -ne 0) { Get-Content "$task/compile.log"; exit $LASTEXITCODE }
$priorTmp = $env:TMP
$priorTemp = $env:TEMP
try {
 # The sandbox permits atomic rename in this authorized workspace. The default
 # OS temp path admitted creation but denied persist; retain that first failure.
 $env:TMP = "$task/harness"
 $env:TEMP = "$task/harness"
 & "$task/harness/checkpoint-tests.exe" checkpoint::tests:: --test-threads=1 *> "$task/tests.log"
 $testExit = $LASTEXITCODE
 Get-Content "$task/tests.log"
 exit $testExit
} finally {
 $env:TMP = $priorTmp
 $env:TEMP = $priorTemp
}
