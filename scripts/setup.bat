@echo off
cd %~dp0


where cargo-binstall >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Installing 'cargo-binstall'...
    cmd /c "powershell -ExecutionPolicy Unrestricted -Command ""Import-Module Microsoft.PowerShell.Utility; Invoke-Expression (Invoke-WebRequest 'https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1').Content"""
    echo.
)


if exist java-spaghetti-gen.exe (
    set /p input="Reinstall 'java-spaghetti-gen'?  [y/N] "
    if "%input%" == "Y" goto UninstallJSGen
    if "%input%" == "y" goto UninstallJSGen
    goto SkipInstallJSGen
    :UninstallJSGen
    echo Deleting previous 'java-spaghetti-gen' executable...
    del /q /f java-spaghetti-gen.exe >nul 2>nul
)

echo Installing 'java-spaghetti-gen'...
echo   - Cloning 'Dirbaio/java-spaghetti'...
rmdir /s /q java-spaghetti >nul 2>nul
git clone https://github.com/Dirbaio/java-spaghetti -n >nul 2>nul
echo   - Checking out commit 'cef9ce16773133fe3c44843b72717d4b678e9f6f'...
pushd java-spaghetti
git checkout cef9ce16773133fe3c44843b72717d4b678e9f6f >nul 2>nul
popd
echo   - Building 'java-spaghetti-gen'...
pushd java-spaghetti\java-spaghetti-gen
cargo build --release
popd
echo   - Moving built executable to '.\scripts'...
copy /y java-spaghetti\target\release\java-spaghetti-gen.exe java-spaghetti-gen.exe >nul 2>nul
echo   - Removing leftovers...
pushd java-spaghetti
cargo clean --quiet
popd
rmdir /s /q java-spaghetti >nul 2>nul
echo.
:SkipInstallJSGen


where form >nul 2>nul
if %ERRORLEVEL% equ 0 (
    set /p input="Reinstall 'form'?  [y/N] "
    if "%input%" == "Y" goto UninstallForm
    if "%input%" == "y" goto UninstallForm
    goto SkipInstallForm
    :UninstallForm
    echo Uninstalling 'form'...
    cargo uninstall form
)

echo Installing 'form'
cargo binstall -y form
:SkipInstallForm

echo Done!
