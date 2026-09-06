; NSIS Installer Script for Sieve by BanguDevClub
!include "MUI2.nsh"

Name "Sieve"
OutFile "/app/dist-build/windows/sieve-setup.exe"
InstallDir "$PROGRAMFILES64\Sieve"
InstallDirRegKey HKLM "Software\BanguDevClub\Sieve" "InstallDir"
RequestExecutionLevel admin

!define MUI_ABORTWARNING
!define MUI_ICON "/app/backend/icons/icon.ico"
!define MUI_UNICON "/app/backend/icons/icon.ico"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath "$INSTDIR"
    File "/app/dist-build/windows/sieve.exe"
    File "/app/backend/icons/icon.ico"
    File "/app/LICENSE"
    File "/app/README.md"

    WriteUninstaller "$INSTDIR\uninstall.exe"

    ; Start Menu Shortcuts
    CreateDirectory "$SMPROGRAMS\Sieve"
    CreateShortcut "$SMPROGRAMS\Sieve\Sieve.lnk" "$INSTDIR\sieve.exe" "" "$INSTDIR\icon.ico"
    CreateShortcut "$SMPROGRAMS\Sieve\Uninstall.lnk" "$INSTDIR\uninstall.exe"

    ; Registry keys for Add/Remove Programs
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sieve" "DisplayName" "Sieve by BanguDevClub"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sieve" "UninstallString" '"$INSTDIR\uninstall.exe"'
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sieve" "DisplayIcon" "$INSTDIR\icon.ico"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sieve" "Publisher" "BanguDevClub"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sieve" "DisplayVersion" "0.1.0"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\sieve.exe"
    Delete "$INSTDIR\icon.ico"
    Delete "$INSTDIR\LICENSE"
    Delete "$INSTDIR\README.md"
    Delete "$INSTDIR\uninstall.exe"

    RMDir /r "$SMPROGRAMS\Sieve"
    RMDir "$INSTDIR"

    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sieve"
    DeleteRegKey HKLM "Software\BanguDevClub\Sieve"
SectionEnd
