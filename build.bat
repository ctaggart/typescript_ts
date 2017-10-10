REM build script for Windows with Visual C++ installed from Visual Studio 2017
REM use vswhere to find Visual C++
REM https://github.com/Microsoft/vswhere/wiki/Find-VC

REM set PATH = "%ProgramFiles(x86)%\Microsoft Visual Studio\Installer"

for /f "usebackq tokens=*" %%i in (`"%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do (
  set VSPATH=%%i
)
REM echo "VSPATH: %VSPATH%"

REM prevent vcvars from changing the directory
set "VSCMD_START_DIR=%CD%"
call "%VSPATH%\VC\Auxiliary\Build\vcvars64.bat"

REM expects ChakraCore to be cloned to a sibling directory
REM https://github.com/Microsoft/ChakraCore
call :NORMALIZEPATH "..\ChakraCore"
set CHAKRA_SOURCE=%RETVAL%
echo "CHAKRA_SOURCE: %CHAKRA_SOURCE%"
set CHAKRA_BUILD=%CHAKRA_SOURCE%\Build\VcBuild\bin\x64_release
cargo build

REM https://stackoverflow.com/a/33404867
:: ========== FUNCTIONS ==========
EXIT /B

:NORMALIZEPATH
  set RETVAL=%~dpfn1
  EXIT /B