@echo off
echo Finding MSVC compiler path...
echo.

REM Try to find cl.exe in typical Visual Studio installation paths
for /d %%d in ("C:\Program Files\Microsoft Visual Studio\2022\*") do (
    if exist "%%d\VC\Tools\MSVC" (
        for /d %%v in ("%%d\VC\Tools\MSVC\*") do (
            if exist "%%v\bin\Hostx86\x86\cl.exe" (
                echo Found: %%v\bin\Hostx86\x86\cl.exe
                echo.
                echo Copy this path and update your c_cpp_properties.json:
                echo "%%v\bin\Hostx86\x86\cl.exe"
                echo.
            )
        )
    )
)

REM Also check for Windows SDK paths
echo Checking Windows SDK paths...
for /d %%d in ("C:\Program Files (x86)\Windows Kits\10\include\*") do (
    if exist "%%d\ucrt" (
        echo Found Windows SDK: %%d
        echo.
    )
)

echo.
echo If no paths were found above, you may need to:
echo 1. Install Visual Studio 2022 with C++ workload
echo 2. Or install Build Tools for Visual Studio 2022
echo 3. Then run this script again
echo.
pause
