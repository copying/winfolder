//! Provides so-called [KNOWNFOLDERID constants](https://msdn.microsoft.com/en-us/library/windows/desktop/dd378457.aspx)
//! of Windows, i.e., the GUIDs associated with standard folders registered with the system as
//! [known folders](https://msdn.microsoft.com/en-us/library/windows/desktop/bb776911.aspx).

use guid::GUID;

/// Default folder [Account Pictures (FOLDERID_AccountPictures)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AccountPictures)
/// GUID(`{008ca0b1-55b4-4c56-b8a8-4de4b299d3be}`)
pub const ACCOUNT_PICTURES : GUID = guid!{"008ca0b1-55b4-4c56-b8a8-4de4b299d3be"};

/// Default folder [Get Programs (FOLDERID_AddNewPrograms)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AddNewPrograms)
/// GUID(`{de61d971-5ebc-4f02-a3a9-6c82895e5c04}`)
pub const ADD_NEW_PROGRAMS : GUID = guid!{"de61d971-5ebc-4f02-a3a9-6c82895e5c04"};

/// Default folder [Administrative Tools (FOLDERID_AdminTools)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AdminTools)
/// GUID(`{724EF170-A42D-4FEF-9F26-B60E846FBA4F}`)
pub const ADMIN_TOOLS : GUID = guid!{"724EF170-A42D-4FEF-9F26-B60E846FBA4F"};

/// Default folder [AppDataDesktop (FOLDERID_AppDataDesktop)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppDataDesktop)
/// GUID(`{B2C5E279-7ADD-439F-B28C-C41FE1BBF672}`)
pub const APP_DATA_DESKTOP : GUID = guid!{"B2C5E279-7ADD-439F-B28C-C41FE1BBF672"};

/// Default folder [AppDataDocuments (FOLDERID_AppDataDocuments)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppDataDocuments)
/// GUID(`{7BE16610-1F7F-44AC-BFF0-83E15F2FFCA1}`)
pub const APP_DATA_DOCUMENTS : GUID = guid!{"7BE16610-1F7F-44AC-BFF0-83E15F2FFCA1"};

/// Default folder [AppDataFavorites (FOLDERID_AppDataFavorites)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppDataFavorites)
/// GUID(`{7CFBEFBC-DE1F-45AA-B843-A542AC536CC9}`)
pub const APP_DATA_FAVORITES : GUID = guid!{"7CFBEFBC-DE1F-45AA-B843-A542AC536CC9"};

/// Default folder [AppDataProgramData (FOLDERID_AppDataProgramData)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppDataProgramData)
/// GUID(`{559D40A3-A036-40FA-AF61-84CB430A4D34}`)
pub const APP_DATA_PROGRAM_DATA : GUID = guid!{"559D40A3-A036-40FA-AF61-84CB430A4D34"};

/// Default folder [Application Shortcuts (FOLDERID_ApplicationShortcuts)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ApplicationShortcuts)
/// GUID(`{A3918781-E5F2-4890-B3D9-A7E54332328C}`)
pub const APPLICATION_SHORTCUTS : GUID = guid!{"A3918781-E5F2-4890-B3D9-A7E54332328C"};

/// Default folder [Applications (FOLDERID_AppsFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppsFolder)
/// GUID(`{1e87508d-89c2-42f0-8a7e-645a0f50ca58}`)
pub const APPS_FOLDER : GUID = guid!{"1e87508d-89c2-42f0-8a7e-645a0f50ca58"};

/// Default folder [Installed Updates (FOLDERID_AppUpdates)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppUpdates)
/// GUID(`{a305ce99-f527-492b-8b1a-7e76fa98d6e4}`)
pub const APP_UPDATES : GUID = guid!{"a305ce99-f527-492b-8b1a-7e76fa98d6e4"};

/// Default folder [Camera Roll (FOLDERID_CameraRoll)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CameraRoll)
/// GUID(`{AB5FB87B-7CE2-4F83-915D-550846C9537B}`)
pub const CAMERA_ROLL : GUID = guid!{"AB5FB87B-7CE2-4F83-915D-550846C9537B"};

/// Default folder [Temporary Burn Folder (FOLDERID_CDBurning)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CDBurning)
/// GUID(`{9E52AB10-F80D-49DF-ACB8-4330F5687855}`)
pub const CDBURNING : GUID = guid!{"9E52AB10-F80D-49DF-ACB8-4330F5687855"};

/// Default folder [Programs and Features (FOLDERID_ChangeRemovePrograms)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ChangeRemovePrograms)
/// GUID(`{df7266ac-9274-4867-8d55-3bd661de872d}`)
pub const CHANGE_REMOVE_PROGRAMS : GUID = guid!{"df7266ac-9274-4867-8d55-3bd661de872d"};

/// Default folder [Administrative Tools (FOLDERID_CommonAdminTools)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonAdminTools)
/// GUID(`{D0384E7D-BAC3-4797-8F14-CBA229B392B5}`)
pub const COMMON_ADMIN_TOOLS : GUID = guid!{"D0384E7D-BAC3-4797-8F14-CBA229B392B5"};

/// Default folder [OEM Links (FOLDERID_CommonOEMLinks)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonOEMLinks)
/// GUID(`{C1BAE2D0-10DF-4334-BEDD-7AA20B227A9D}`)
pub const COMMON_OEMLINKS : GUID = guid!{"C1BAE2D0-10DF-4334-BEDD-7AA20B227A9D"};

/// Default folder [Programs (FOLDERID_CommonPrograms)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonPrograms)
/// GUID(`{0139D44E-6AFE-49F2-8690-3DAFCAE6FFB8}`)
pub const COMMON_PROGRAMS : GUID = guid!{"0139D44E-6AFE-49F2-8690-3DAFCAE6FFB8"};

/// Default folder [Start Menu (FOLDERID_CommonStartMenu)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonStartMenu)
/// GUID(`{A4115719-D62E-491D-AA7C-E74B8BE3B067}`)
pub const COMMON_START_MENU : GUID = guid!{"A4115719-D62E-491D-AA7C-E74B8BE3B067"};

/// Default folder [Startup (FOLDERID_CommonStartup)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonStartup)
/// GUID(`{82A5EA35-D9CD-47C5-9629-E15D2F714E6E}`)
pub const COMMON_STARTUP : GUID = guid!{"82A5EA35-D9CD-47C5-9629-E15D2F714E6E"};

/// Default folder [Templates (FOLDERID_CommonTemplates)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonTemplates)
/// GUID(`{B94237E7-57AC-4347-9151-B08C6C32D1F7}`)
pub const COMMON_TEMPLATES : GUID = guid!{"B94237E7-57AC-4347-9151-B08C6C32D1F7"};

/// Default folder [Computer (FOLDERID_ComputerFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ComputerFolder)
/// GUID(`{0AC0837C-BBF8-452A-850D-79D08E667CA7}`)
pub const COMPUTER_FOLDER : GUID = guid!{"0AC0837C-BBF8-452A-850D-79D08E667CA7"};

/// Default folder [Conflicts (FOLDERID_ConflictFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ConflictFolder)
/// GUID(`{4bfefb45-347d-4006-a5be-ac0cb0567192}`)
pub const CONFLICT_FOLDER : GUID = guid!{"4bfefb45-347d-4006-a5be-ac0cb0567192"};

/// Default folder [Network Connections (FOLDERID_ConnectionsFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ConnectionsFolder)
/// GUID(`{6F0CD92B-2E97-45D1-88FF-B0D186B8DEDD}`)
pub const CONNECTIONS_FOLDER : GUID = guid!{"6F0CD92B-2E97-45D1-88FF-B0D186B8DEDD"};

/// Default folder [Contacts (FOLDERID_Contacts)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Contacts)
/// GUID(`{56784854-C6CB-462b-8169-88E350ACB882}`)
pub const CONTACTS : GUID = guid!{"56784854-C6CB-462b-8169-88E350ACB882"};

/// Default folder [Control Panel (FOLDERID_ControlPanelFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ControlPanelFolder)
/// GUID(`{82A74AEB-AEB4-465C-A014-D097EE346D63}`)
pub const CONTROL_PANEL_FOLDER : GUID = guid!{"82A74AEB-AEB4-465C-A014-D097EE346D63"};

/// Default folder [Cookies (FOLDERID_Cookies)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Cookies)
/// GUID(`{2B0F765D-C0E9-4171-908E-08A611B84FF6}`)
pub const COOKIES : GUID = guid!{"2B0F765D-C0E9-4171-908E-08A611B84FF6"};

/// Default folder [Desktop (FOLDERID_Desktop)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Desktop)
/// GUID(`{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}`)
pub const DESKTOP : GUID = guid!{"B4BFCC3A-DB2C-424C-B029-7FE99A87C641"};

/// Default folder [DeviceMetadataStore (FOLDERID_DeviceMetadataStore)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_DeviceMetadataStore)
/// GUID(`{5CE4A5E9-E4EB-479D-B89F-130C02886155}`)
pub const DEVICE_METADATA_STORE : GUID = guid!{"5CE4A5E9-E4EB-479D-B89F-130C02886155"};

/// Default folder [Documents (FOLDERID_Documents)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Documents)
/// GUID(`{FDD39AD0-238F-46AF-ADB4-6C85480369C7}`)
pub const DOCUMENTS : GUID = guid!{"FDD39AD0-238F-46AF-ADB4-6C85480369C7"};

/// Default folder [Documents (FOLDERID_DocumentsLibrary)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_DocumentsLibrary)
/// GUID(`{7B0DB17D-9CD2-4A93-9733-46CC89022E7C}`)
pub const DOCUMENTS_LIBRARY : GUID = guid!{"7B0DB17D-9CD2-4A93-9733-46CC89022E7C"};

/// Default folder [Downloads (FOLDERID_Downloads)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Downloads)
/// GUID(`{374DE290-123F-4565-9164-39C4925E467B}`)
pub const DOWNLOADS : GUID = guid!{"374DE290-123F-4565-9164-39C4925E467B"};

/// Default folder [Favorites (FOLDERID_Favorites)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Favorites)
/// GUID(`{1777F761-68AD-4D8A-87BD-30B759FA33DD}`)
pub const FAVORITES : GUID = guid!{"1777F761-68AD-4D8A-87BD-30B759FA33DD"};

/// Default folder [Fonts (FOLDERID_Fonts)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Fonts)
/// GUID(`{FD228CB7-AE11-4AE3-864C-16F3910AB8FE}`)
pub const FONTS : GUID = guid!{"FD228CB7-AE11-4AE3-864C-16F3910AB8FE"};

/// Default folder [Games (FOLDERID_Games)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Games)
/// GUID(`{CAC52C1A-B53D-4edc-92D7-6B2E8AC19434}`)
pub const GAMES : GUID = guid!{"CAC52C1A-B53D-4edc-92D7-6B2E8AC19434"};

/// Default folder [GameExplorer (FOLDERID_GameTasks)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_GameTasks)
/// GUID(`{054FAE61-4DD8-4787-80B6-090220C4B700}`)
pub const GAME_TASKS : GUID = guid!{"054FAE61-4DD8-4787-80B6-090220C4B700"};

/// Default folder [History (FOLDERID_History)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_History)
/// GUID(`{D9DC8A3B-B784-432E-A781-5A1130A75963}`)
pub const HISTORY : GUID = guid!{"D9DC8A3B-B784-432E-A781-5A1130A75963"};

/// Default folder [Homegroup (FOLDERID_HomeGroup)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_HomeGroup)
/// GUID(`{52528A6B-B9E3-4ADD-B60D-588C2DBA842D}`)
pub const HOME_GROUP : GUID = guid!{"52528A6B-B9E3-4ADD-B60D-588C2DBA842D"};

/// Default folder [The user's username (%USERNAME (FOLDERID_HomeGroupCurrentUser)%)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_HomeGroupCurrentUser)
/// GUID(`{9B74B6A3-0DFD-4f11-9E78-5F7800F2E772}`)
pub const HOME_GROUP_CURRENT_USER : GUID = guid!{"9B74B6A3-0DFD-4f11-9E78-5F7800F2E772"};

/// Default folder [ImplicitAppShortcuts (FOLDERID_ImplicitAppShortcuts)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ImplicitAppShortcuts)
/// GUID(`{BCB5256F-79F6-4CEE-B725-DC34E402FD46}`)
pub const IMPLICIT_APP_SHORTCUTS : GUID = guid!{"BCB5256F-79F6-4CEE-B725-DC34E402FD46"};

/// Default folder [Temporary Internet Files (FOLDERID_InternetCache)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_InternetCache)
/// GUID(`{352481E8-33BE-4251-BA85-6007CAEDCF9D}`)
pub const INTERNET_CACHE : GUID = guid!{"352481E8-33BE-4251-BA85-6007CAEDCF9D"};

/// Default folder [The Internet (FOLDERID_InternetFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_InternetFolder)
/// GUID(`{4D9F7874-4E0C-4904-967B-40B0D20C3E4B}`)
pub const INTERNET_FOLDER : GUID = guid!{"4D9F7874-4E0C-4904-967B-40B0D20C3E4B"};

/// Default folder [Libraries (FOLDERID_Libraries)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Libraries)
/// GUID(`{1B3EA5DC-B587-4786-B4EF-BD1DC332AEAE}`)
pub const LIBRARIES : GUID = guid!{"1B3EA5DC-B587-4786-B4EF-BD1DC332AEAE"};

/// Default folder [Links (FOLDERID_Links)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Links)
/// GUID(`{bfb9d5e0-c6a9-404c-b2b2-ae6db6af4968}`)
pub const LINKS : GUID = guid!{"bfb9d5e0-c6a9-404c-b2b2-ae6db6af4968"};

/// Default folder [Local (FOLDERID_LocalAppData)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalAppData)
/// GUID(`{F1B32785-6FBA-4FCF-9D55-7B8E7F157091}`)
pub const LOCAL_APP_DATA : GUID = guid!{"F1B32785-6FBA-4FCF-9D55-7B8E7F157091"};

/// Default folder [LocalLow (FOLDERID_LocalAppDataLow)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalAppDataLow)
/// GUID(`{A520A1A4-1780-4FF6-BD18-167343C5AF16}`)
pub const LOCAL_APP_DATA_LOW : GUID = guid!{"A520A1A4-1780-4FF6-BD18-167343C5AF16"};

/// Default folder [None (FOLDERID_LocalizedResourcesDir)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalizedResourcesDir)
/// GUID(`{2A00375E-224C-49DE-B8D1-440DF7EF3DDC}`)
pub const LOCALIZED_RESOURCES_DIR : GUID = guid!{"2A00375E-224C-49DE-B8D1-440DF7EF3DDC"};

/// Default folder [Music (FOLDERID_Music)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Music)
/// GUID(`{4BD8D571-6D19-48D3-BE97-422220080E43}`)
pub const MUSIC : GUID = guid!{"4BD8D571-6D19-48D3-BE97-422220080E43"};

/// Default folder [Music (FOLDERID_MusicLibrary)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_MusicLibrary)
/// GUID(`{2112AB0A-C86A-4FFE-A368-0DE96E47012E}`)
pub const MUSIC_LIBRARY : GUID = guid!{"2112AB0A-C86A-4FFE-A368-0DE96E47012E"};

/// Default folder [Network Shortcuts (FOLDERID_NetHood)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_NetHood)
/// GUID(`{C5ABBF53-E17F-4121-8900-86626FC2C973}`)
pub const NET_HOOD : GUID = guid!{"C5ABBF53-E17F-4121-8900-86626FC2C973"};

/// Default folder [Network (FOLDERID_NetworkFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_NetworkFolder)
/// GUID(`{D20BEEC4-5CA8-4905-AE3B-BF251EA09B53}`)
pub const NETWORK_FOLDER : GUID = guid!{"D20BEEC4-5CA8-4905-AE3B-BF251EA09B53"};

/// Default folder [3D Objects (FOLDERID_Objects3D)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Objects3D)
/// GUID(`{31C0DD25-9439-4F12-BF41-7FF4EDA38722}`)
pub const OBJECTS3D : GUID = guid!{"31C0DD25-9439-4F12-BF41-7FF4EDA38722"};

/// Default folder [Original Images (FOLDERID_OriginalImages)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_OriginalImages)
/// GUID(`{2C36C0AA-5812-4b87-BFD0-4CD0DFB19B39}`)
pub const ORIGINAL_IMAGES : GUID = guid!{"2C36C0AA-5812-4b87-BFD0-4CD0DFB19B39"};

/// Default folder [Slide Shows (FOLDERID_PhotoAlbums)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PhotoAlbums)
/// GUID(`{69D2CF90-FC33-4FB7-9A0C-EBB0F0FCB43C}`)
pub const PHOTO_ALBUMS : GUID = guid!{"69D2CF90-FC33-4FB7-9A0C-EBB0F0FCB43C"};

/// Default folder [Pictures (FOLDERID_PicturesLibrary)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PicturesLibrary)
/// GUID(`{A990AE9F-A03B-4E80-94BC-9912D7504104}`)
pub const PICTURES_LIBRARY : GUID = guid!{"A990AE9F-A03B-4E80-94BC-9912D7504104"};

/// Default folder [Pictures (FOLDERID_Pictures)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Pictures)
/// GUID(`{33E28130-4E1E-4676-835A-98395C3BC3BB}`)
pub const PICTURES : GUID = guid!{"33E28130-4E1E-4676-835A-98395C3BC3BB"};

/// Default folder [Playlists (FOLDERID_Playlists)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Playlists)
/// GUID(`{DE92C1C7-837F-4F69-A3BB-86E631204A23}`)
pub const PLAYLISTS : GUID = guid!{"DE92C1C7-837F-4F69-A3BB-86E631204A23"};

/// Default folder [Printers (FOLDERID_PrintersFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PrintersFolder)
/// GUID(`{76FC4E2D-D6AD-4519-A663-37BD56068185}`)
pub const PRINTERS_FOLDER : GUID = guid!{"76FC4E2D-D6AD-4519-A663-37BD56068185"};

/// Default folder [Printer Shortcuts (FOLDERID_PrintHood)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PrintHood)
/// GUID(`{9274BD8D-CFD1-41C3-B35E-B13F55A758F4}`)
pub const PRINT_HOOD : GUID = guid!{"9274BD8D-CFD1-41C3-B35E-B13F55A758F4"};

/// Default folder [The user's username (%USERNAME (FOLDERID_Profile)%)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Profile)
/// GUID(`{5E6C858F-0E22-4760-9AFE-EA3317B67173}`)
pub const PROFILE : GUID = guid!{"5E6C858F-0E22-4760-9AFE-EA3317B67173"};

/// Default folder [ProgramData (FOLDERID_ProgramData)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramData)
/// GUID(`{62AB5D82-FDC1-4DC3-A9DD-070D1D495D97}`)
pub const PROGRAM_DATA : GUID = guid!{"62AB5D82-FDC1-4DC3-A9DD-070D1D495D97"};

/// Default folder [Program Files (FOLDERID_ProgramFiles)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFiles)
/// GUID(`{905e63b6-c1bf-494e-b29c-65b732d3d21a}`)
pub const PROGRAM_FILES : GUID = guid!{"905e63b6-c1bf-494e-b29c-65b732d3d21a"};

/// Default folder [Program Files (FOLDERID_ProgramFilesX64)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFilesX64)
/// GUID(`{6D809377-6AF0-444b-8957-A3773F02200E}`)
pub const PROGRAM_FILES_X64 : GUID = guid!{"6D809377-6AF0-444b-8957-A3773F02200E"};

/// Default folder [Program Files (FOLDERID_ProgramFilesX86)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFilesX86)
/// GUID(`{7C5A40EF-A0FB-4BFC-874A-C0F2E0B9FA8E}`)
pub const PROGRAM_FILES_X86 : GUID = guid!{"7C5A40EF-A0FB-4BFC-874A-C0F2E0B9FA8E"};

/// Default folder [Common Files (FOLDERID_ProgramFilesCommon)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFilesCommon)
/// GUID(`{F7F1ED05-9F6D-47A2-AAAE-29D317C6F066}`)
pub const PROGRAM_FILES_COMMON : GUID = guid!{"F7F1ED05-9F6D-47A2-AAAE-29D317C6F066"};

/// Default folder [Common Files (FOLDERID_ProgramFilesCommonX64)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFilesCommonX64)
/// GUID(`{6365D5A7-0F0D-45E5-87F6-0DA56B6A4F7D}`)
pub const PROGRAM_FILES_COMMON_X64 : GUID = guid!{"6365D5A7-0F0D-45E5-87F6-0DA56B6A4F7D"};

/// Default folder [Common Files (FOLDERID_ProgramFilesCommonX86)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFilesCommonX86)
/// GUID(`{DE974D24-D9C6-4D3E-BF91-F4455120B917}`)
pub const PROGRAM_FILES_COMMON_X86 : GUID = guid!{"DE974D24-D9C6-4D3E-BF91-F4455120B917"};

/// Default folder [Programs (FOLDERID_Programs)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Programs)
/// GUID(`{A77F5D77-2E2B-44C3-A6A2-ABA601054A51}`)
pub const PROGRAMS : GUID = guid!{"A77F5D77-2E2B-44C3-A6A2-ABA601054A51"};

/// Default folder [Public (FOLDERID_Public)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Public)
/// GUID(`{DFDF76A2-C82A-4D63-906A-5644AC457385}`)
pub const PUBLIC : GUID = guid!{"DFDF76A2-C82A-4D63-906A-5644AC457385"};

/// Default folder [Public Desktop (FOLDERID_PublicDesktop)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicDesktop)
/// GUID(`{C4AA340D-F20F-4863-AFEF-F87EF2E6BA25}`)
pub const PUBLIC_DESKTOP : GUID = guid!{"C4AA340D-F20F-4863-AFEF-F87EF2E6BA25"};

/// Default folder [Public Documents (FOLDERID_PublicDocuments)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicDocuments)
/// GUID(`{ED4824AF-DCE4-45A8-81E2-FC7965083634}`)
pub const PUBLIC_DOCUMENTS : GUID = guid!{"ED4824AF-DCE4-45A8-81E2-FC7965083634"};

/// Default folder [Public Downloads (FOLDERID_PublicDownloads)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicDownloads)
/// GUID(`{3D644C9B-1FB8-4f30-9B45-F670235F79C0}`)
pub const PUBLIC_DOWNLOADS : GUID = guid!{"3D644C9B-1FB8-4f30-9B45-F670235F79C0"};

/// Default folder [GameExplorer (FOLDERID_PublicGameTasks)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicGameTasks)
/// GUID(`{DEBF2536-E1A8-4c59-B6A2-414586476AEA}`)
pub const PUBLIC_GAME_TASKS : GUID = guid!{"DEBF2536-E1A8-4c59-B6A2-414586476AEA"};

/// Default folder [Libraries (FOLDERID_PublicLibraries)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicLibraries)
/// GUID(`{48DAF80B-E6CF-4F4E-B800-0E69D84EE384}`)
pub const PUBLIC_LIBRARIES : GUID = guid!{"48DAF80B-E6CF-4F4E-B800-0E69D84EE384"};

/// Default folder [Public Music (FOLDERID_PublicMusic)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicMusic)
/// GUID(`{3214FAB5-9757-4298-BB61-92A9DEAA44FF}`)
pub const PUBLIC_MUSIC : GUID = guid!{"3214FAB5-9757-4298-BB61-92A9DEAA44FF"};

/// Default folder [Public Pictures (FOLDERID_PublicPictures)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicPictures)
/// GUID(`{B6EBFB86-6907-413C-9AF7-4FC2ABF07CC5}`)
pub const PUBLIC_PICTURES : GUID = guid!{"B6EBFB86-6907-413C-9AF7-4FC2ABF07CC5"};

/// Default folder [Ringtones (FOLDERID_PublicRingtones)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicRingtones)
/// GUID(`{E555AB60-153B-4D17-9F04-A5FE99FC15EC}`)
pub const PUBLIC_RINGTONES : GUID = guid!{"E555AB60-153B-4D17-9F04-A5FE99FC15EC"};

/// Default folder [Public Account Pictures (FOLDERID_PublicUserTiles)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicUserTiles)
/// GUID(`{0482af6c-08f1-4c34-8c90-e17ec98b1e17}`)
pub const PUBLIC_USER_TILES : GUID = guid!{"0482af6c-08f1-4c34-8c90-e17ec98b1e17"};

/// Default folder [Public Videos (FOLDERID_PublicVideos)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicVideos)
/// GUID(`{2400183A-6185-49FB-A2D8-4A392A602BA3}`)
pub const PUBLIC_VIDEOS : GUID = guid!{"2400183A-6185-49FB-A2D8-4A392A602BA3"};

/// Default folder [Quick Launch (FOLDERID_QuickLaunch)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_QuickLaunch)
/// GUID(`{52a4f021-7b75-48a9-9f6b-4b87a210bc8f}`)
pub const QUICK_LAUNCH : GUID = guid!{"52a4f021-7b75-48a9-9f6b-4b87a210bc8f"};

/// Default folder [Recent Items (FOLDERID_Recent)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Recent)
/// GUID(`{AE50C081-EBD2-438A-8655-8A092E34987A}`)
pub const RECENT : GUID = guid!{"AE50C081-EBD2-438A-8655-8A092E34987A"};

/// Default folder [Recorded TV (FOLDERID_RecordedTVLibrary)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RecordedTVLibrary)
/// GUID(`{1A6FDBA2-F42D-4358-A798-B74D745926C5}`)
pub const RECORDED_TVLIBRARY : GUID = guid!{"1A6FDBA2-F42D-4358-A798-B74D745926C5"};

/// Default folder [Recycle Bin (FOLDERID_RecycleBinFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RecycleBinFolder)
/// GUID(`{B7534046-3ECB-4C18-BE4E-64CD4CB7D6AC}`)
pub const RECYCLE_BIN_FOLDER : GUID = guid!{"B7534046-3ECB-4C18-BE4E-64CD4CB7D6AC"};

/// Default folder [Resources (FOLDERID_ResourceDir)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ResourceDir)
/// GUID(`{8AD10C31-2ADB-4296-A8F7-E4701232C972}`)
pub const RESOURCE_DIR : GUID = guid!{"8AD10C31-2ADB-4296-A8F7-E4701232C972"};

/// Default folder [Ringtones (FOLDERID_Ringtones)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Ringtones)
/// GUID(`{C870044B-F49E-4126-A9C3-B52A1FF411E8}`)
pub const RINGTONES : GUID = guid!{"C870044B-F49E-4126-A9C3-B52A1FF411E8"};

/// Default folder [Roaming (FOLDERID_RoamingAppData)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RoamingAppData)
/// GUID(`{3EB685DB-65F9-4CF6-A03A-E3EF65729F3D}`)
pub const ROAMING_APP_DATA : GUID = guid!{"3EB685DB-65F9-4CF6-A03A-E3EF65729F3D"};

/// Default folder [RoamedTileImages (FOLDERID_RoamedTileImages)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RoamedTileImages)
/// GUID(`{AAA8D5A5-F1D6-4259-BAA8-78E7EF60835E}`)
pub const ROAMED_TILE_IMAGES : GUID = guid!{"AAA8D5A5-F1D6-4259-BAA8-78E7EF60835E"};

/// Default folder [RoamingTiles (FOLDERID_RoamingTiles)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RoamingTiles)
/// GUID(`{00BCFC5A-ED94-4e48-96A1-3F6217F21990}`)
pub const ROAMING_TILES : GUID = guid!{"00BCFC5A-ED94-4e48-96A1-3F6217F21990"};

/// Default folder [Sample Music (FOLDERID_SampleMusic)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SampleMusic)
/// GUID(`{B250C668-F57D-4EE1-A63C-290EE7D1AA1F}`)
pub const SAMPLE_MUSIC : GUID = guid!{"B250C668-F57D-4EE1-A63C-290EE7D1AA1F"};

/// Default folder [Sample Pictures (FOLDERID_SamplePictures)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SamplePictures)
/// GUID(`{C4900540-2379-4C75-844B-64E6FAF8716B}`)
pub const SAMPLE_PICTURES : GUID = guid!{"C4900540-2379-4C75-844B-64E6FAF8716B"};

/// Default folder [Sample Playlists (FOLDERID_SamplePlaylists)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SamplePlaylists)
/// GUID(`{15CA69B3-30EE-49C1-ACE1-6B5EC372AFB5}`)
pub const SAMPLE_PLAYLISTS : GUID = guid!{"15CA69B3-30EE-49C1-ACE1-6B5EC372AFB5"};

/// Default folder [Sample Videos (FOLDERID_SampleVideos)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SampleVideos)
/// GUID(`{859EAD94-2E85-48AD-A71A-0969CB56A6CD}`)
pub const SAMPLE_VIDEOS : GUID = guid!{"859EAD94-2E85-48AD-A71A-0969CB56A6CD"};

/// Default folder [Saved Games (FOLDERID_SavedGames)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SavedGames)
/// GUID(`{4C5C32FF-BB9D-43b0-B5B4-2D72E54EAAA4}`)
pub const SAVED_GAMES : GUID = guid!{"4C5C32FF-BB9D-43b0-B5B4-2D72E54EAAA4"};

/// Default folder [Saved Pictures (FOLDERID_SavedPictures)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SavedPictures)
/// GUID(`{3B193882-D3AD-4eab-965A-69829D1FB59F}`)
pub const SAVED_PICTURES : GUID = guid!{"3B193882-D3AD-4eab-965A-69829D1FB59F"};

/// Default folder [Saved Pictures Library (FOLDERID_SavedPicturesLibrary)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SavedPicturesLibrary)
/// GUID(`{E25B5812-BE88-4bd9-94B0-29233477B6C3}`)
pub const SAVED_PICTURES_LIBRARY : GUID = guid!{"E25B5812-BE88-4bd9-94B0-29233477B6C3"};

/// Default folder [Searches (FOLDERID_SavedSearches)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SavedSearches)
/// GUID(`{7d1d3a04-debb-4115-95cf-2f29da2920da}`)
pub const SAVED_SEARCHES : GUID = guid!{"7d1d3a04-debb-4115-95cf-2f29da2920da"};

/// Default folder [Screenshots (FOLDERID_Screenshots)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Screenshots)
/// GUID(`{b7bede81-df94-4682-a7d8-57a52620b86f}`)
pub const SCREENSHOTS : GUID = guid!{"b7bede81-df94-4682-a7d8-57a52620b86f"};

/// Default folder [Offline Files (FOLDERID_SEARCH_CSC)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SEARCH_CSC)
/// GUID(`{ee32e446-31ca-4aba-814f-a5ebd2fd6d5e}`)
pub const SEARCH_CSC : GUID = guid!{"ee32e446-31ca-4aba-814f-a5ebd2fd6d5e"};

/// Default folder [History (FOLDERID_SearchHistory)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SearchHistory)
/// GUID(`{0D4C3DB6-03A3-462F-A0E6-08924C41B5D4}`)
pub const SEARCH_HISTORY : GUID = guid!{"0D4C3DB6-03A3-462F-A0E6-08924C41B5D4"};

/// Default folder [Search Results (FOLDERID_SearchHome)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SearchHome)
/// GUID(`{190337d1-b8ca-4121-a639-6d472d16972a}`)
pub const SEARCH_HOME : GUID = guid!{"190337d1-b8ca-4121-a639-6d472d16972a"};

/// Default folder [Microsoft Office Outlook (FOLDERID_SEARCH_MAPI)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SEARCH_MAPI)
/// GUID(`{98ec0e18-2098-4d44-8644-66979315a281}`)
pub const SEARCH_MAPI : GUID = guid!{"98ec0e18-2098-4d44-8644-66979315a281"};

/// Default folder [Templates (FOLDERID_SearchTemplates)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SearchTemplates)
/// GUID(`{7E636BFE-DFA9-4D5E-B456-D7B39851D8A9}`)
pub const SEARCH_TEMPLATES : GUID = guid!{"7E636BFE-DFA9-4D5E-B456-D7B39851D8A9"};

/// Default folder [SendTo (FOLDERID_SendTo)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SendTo)
/// GUID(`{8983036C-27C0-404B-8F08-102D10DCFD74}`)
pub const SEND_TO : GUID = guid!{"8983036C-27C0-404B-8F08-102D10DCFD74"};

/// Default folder [Gadgets (FOLDERID_SidebarDefaultParts)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SidebarDefaultParts)
/// GUID(`{7B396E54-9EC5-4300-BE0A-2482EBAE1A26}`)
pub const SIDEBAR_DEFAULT_PARTS : GUID = guid!{"7B396E54-9EC5-4300-BE0A-2482EBAE1A26"};

/// Default folder [Gadgets (FOLDERID_SidebarParts)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SidebarParts)
/// GUID(`{A75D362E-50FC-4fb7-AC2C-A8BEAA314493}`)
pub const SIDEBAR_PARTS : GUID = guid!{"A75D362E-50FC-4fb7-AC2C-A8BEAA314493"};

/// Default folder [OneDrive (FOLDERID_SkyDrive)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SkyDrive)
/// GUID(`{A52BBA46-E9E1-435f-B3D9-28DAA648C0F6}`)
pub const SKY_DRIVE : GUID = guid!{"A52BBA46-E9E1-435f-B3D9-28DAA648C0F6"};

/// Default folder [Camera Roll (FOLDERID_SkyDriveCameraRoll)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SkyDriveCameraRoll)
/// GUID(`{767E6811-49CB-4273-87C2-20F355E1085B}`)
pub const SKY_DRIVE_CAMERA_ROLL : GUID = guid!{"767E6811-49CB-4273-87C2-20F355E1085B"};

/// Default folder [Documents (FOLDERID_SkyDriveDocuments)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SkyDriveDocuments)
/// GUID(`{24D89E24-2F19-4534-9DDE-6A6671FBB8FE}`)
pub const SKY_DRIVE_DOCUMENTS : GUID = guid!{"24D89E24-2F19-4534-9DDE-6A6671FBB8FE"};

/// Default folder [Pictures (FOLDERID_SkyDrivePictures)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SkyDrivePictures)
/// GUID(`{339719B5-8C47-4894-94C2-D8F77ADD44A6}`)
pub const SKY_DRIVE_PICTURES : GUID = guid!{"339719B5-8C47-4894-94C2-D8F77ADD44A6"};

/// Default folder [Start Menu (FOLDERID_StartMenu)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_StartMenu)
/// GUID(`{625B53C3-AB48-4EC1-BA1F-A1EF4146FC19}`)
pub const START_MENU : GUID = guid!{"625B53C3-AB48-4EC1-BA1F-A1EF4146FC19"};

/// Default folder [Startup (FOLDERID_Startup)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Startup)
/// GUID(`{B97D20BB-F46A-4C97-BA10-5E3608430854}`)
pub const STARTUP : GUID = guid!{"B97D20BB-F46A-4C97-BA10-5E3608430854"};

/// Default folder [Sync Center (FOLDERID_SyncManagerFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SyncManagerFolder)
/// GUID(`{43668BF8-C14E-49B2-97C9-747784D784B7}`)
pub const SYNC_MANAGER_FOLDER : GUID = guid!{"43668BF8-C14E-49B2-97C9-747784D784B7"};

/// Default folder [Sync Results (FOLDERID_SyncResultsFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SyncResultsFolder)
/// GUID(`{289a9a43-be44-4057-a41b-587a76d7e7f9}`)
pub const SYNC_RESULTS_FOLDER : GUID = guid!{"289a9a43-be44-4057-a41b-587a76d7e7f9"};

/// Default folder [Sync Setup (FOLDERID_SyncSetupFolder)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SyncSetupFolder)
/// GUID(`{0F214138-B1D3-4a90-BBA9-27CBC0C5389A}`)
pub const SYNC_SETUP_FOLDER : GUID = guid!{"0F214138-B1D3-4a90-BBA9-27CBC0C5389A"};

/// Default folder [System32 (FOLDERID_System)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_System)
/// GUID(`{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}`)
pub const SYSTEM : GUID = guid!{"1AC14E77-02E7-4E5D-B744-2EB1AE5198B7"};

/// Default folder [System32 (FOLDERID_SystemX86)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SystemX86)
/// GUID(`{D65231B0-B2F1-4857-A4CE-A8E7C6EA7D27}`)
pub const SYSTEM_X86 : GUID = guid!{"D65231B0-B2F1-4857-A4CE-A8E7C6EA7D27"};

/// Default folder [Templates (FOLDERID_Templates)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Templates)
/// GUID(`{A63293E8-664E-48DB-A079-DF759E0509F7}`)
pub const TEMPLATES : GUID = guid!{"A63293E8-664E-48DB-A079-DF759E0509F7"};

/// Default folder [User Pinned (FOLDERID_UserPinned)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UserPinned)
/// GUID(`{9E3995AB-1F9C-4F13-B827-48B24B6C7174}`)
pub const USER_PINNED : GUID = guid!{"9E3995AB-1F9C-4F13-B827-48B24B6C7174"};

/// Default folder [Users (FOLDERID_UserProfiles)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UserProfiles)
/// GUID(`{0762D272-C50A-4BB0-A382-697DCD729B80}`)
pub const USER_PROFILES : GUID = guid!{"0762D272-C50A-4BB0-A382-697DCD729B80"};

/// Default folder [Programs (FOLDERID_UserProgramFiles)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UserProgramFiles)
/// GUID(`{5CD7AEE2-2219-4A67-B85D-6C9CE15660CB}`)
pub const USER_PROGRAM_FILES : GUID = guid!{"5CD7AEE2-2219-4A67-B85D-6C9CE15660CB"};

/// Default folder [Programs (FOLDERID_UserProgramFilesCommon)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UserProgramFilesCommon)
/// GUID(`{BCBD3057-CA5C-4622-B42D-BC56DB0AE516}`)
pub const USER_PROGRAM_FILES_COMMON : GUID = guid!{"BCBD3057-CA5C-4622-B42D-BC56DB0AE516"};

/// Default folder [The user's full name (for instance, Jean Philippe Bagel) entered when the user account was created (FOLDERID_UsersFiles).](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UsersFiles)
/// GUID(`{f3ce0f7c-4901-4acc-8648-d5d44b04ef8f}`)
pub const USERS_FILES : GUID = guid!{"f3ce0f7c-4901-4acc-8648-d5d44b04ef8f"};

/// Default folder [Libraries (FOLDERID_UsersLibraries)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UsersLibraries)
/// GUID(`{A302545D-DEFF-464b-ABE8-61C8648D939B}`)
pub const USERS_LIBRARIES : GUID = guid!{"A302545D-DEFF-464b-ABE8-61C8648D939B"};

/// Default folder [Videos (FOLDERID_Videos)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Videos)
/// GUID(`{18989B1D-99B5-455B-841C-AB7C74E4DDFC}`)
pub const VIDEOS : GUID = guid!{"18989B1D-99B5-455B-841C-AB7C74E4DDFC"};

/// Default folder [Videos (FOLDERID_VideosLibrary)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_VideosLibrary)
/// GUID(`{491E922F-5643-4AF4-A7EB-4E7A138D8174}`)
pub const VIDEOS_LIBRARY : GUID = guid!{"491E922F-5643-4AF4-A7EB-4E7A138D8174"};

/// Default folder [Windows  (FOLDERID_Windows)](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Windows)
/// GUID(`{F38BF404-1D43-42F2-9305-67DE0B28FC23}`)
pub const WINDOWS : GUID = guid!{"F38BF404-1D43-42F2-9305-67DE0B28FC23"};
