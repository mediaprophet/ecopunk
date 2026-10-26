$ErrorActionPreference='Stop'
$task=$PSScriptRoot
$deps='C:/github/ecopunk/target/debug/deps'
$arguments=@('--test','--edition=2021',"$task/harness/lib.rs",'--extern',"qualia_core_db=$deps/libqualia_core_db.rlib",'--extern',"tempfile=$deps/libtempfile-6092d433ff7bb176.rlib",'--extern',"quick_xml=$deps/libquick_xml-22d41287daaf3771.rlib",'-L',"dependency=$deps",'-L','native=C:/github/ecopunk/target/debug/build/blake3-da8ba380479449f1/out','-L','native=C:/github/ecopunk/target/debug/build/ring-b53baf53c685dea7/out','-L','native=C:/github/ecopunk/target/debug/build/zstd-sys-ddb15c1df437b75d/out','-L','native=C:/Users/tim_h/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/winapi-x86_64-pc-windows-gnu-0.4.0/lib','-L','native=C:/github/qualiaDB/vendor/directml/bin/x64-win','-L','native=C:/Users/tim_h/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/windows_x86_64_gnu-0.52.6/lib','-o',"$task/harness/graph-repairs-tests.exe")
& rustc @arguments *> "$task/compile.log"
if($LASTEXITCODE -ne 0){Get-Content "$task/compile.log";exit $LASTEXITCODE}
$priorTmp=$env:TMP;$priorTemp=$env:TEMP
try{
 $env:TMP="$task/harness";$env:TEMP="$task/harness"
 $filters=@('candidate_lexicon::tests','sparql_library::sparql_mm::tests::c2pa','daemon_query::tests::sparql_local_service_executes_inner','sparql_library::sparql_grammar::pattern::tests::only_exact','studio_store::cml::tests','solid_ldp::export_q42::tests','graph_repairs')
 $failed=0
 foreach($filter in $filters){
  & "$task/harness/graph-repairs-tests.exe" $filter --nocapture --test-threads=1 *>> "$task/tests.log"
  if($LASTEXITCODE -ne 0){$failed++}
 }
 Get-Content "$task/tests.log"
 if($failed -ne 0){exit 1}
}finally{$env:TMP=$priorTmp;$env:TEMP=$priorTemp}


