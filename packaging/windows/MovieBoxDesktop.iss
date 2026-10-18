#define AppName "MovieBox Desktop"
#define AppVersion "0.1.0"
#define AppPublisher "MovieBox Desktop"
#define AppExeName "moviebox-desktop.exe"

[Setup]
AppId={{B32D5C9D-0D4D-4B0E-9AC7-1AB1E18B61D2}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
DefaultDirName={autopf}\MovieBox Desktop
DefaultGroupName={#AppName}
UninstallDisplayIcon={app}\{#AppExeName}
OutputDir=..\..\dist
OutputBaseFilename=MovieBox-Desktop-Setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=lowest

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional shortcuts:"; Flags: unchecked

[Files]
Source: "..\..\target\release\moviebox-desktop.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\dist\README.txt"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\MovieBox Desktop"; Filename: "{app}\{#AppExeName}"
Name: "{autodesktop}\MovieBox Desktop"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExeName}"; Description: "Launch MovieBox Desktop"; Flags: postinstall nowait skipifsilent
