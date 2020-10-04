use std::path::PathBuf;
use super::id::*;
use super::known_path;

// Added before the declaration and the impl with `id` to improve readability of the file (the enum is long).
impl Folder {
    /// Returns the path for this known folder on this system.
    ///
    /// This function provides the functionality of the standard Windows
    /// [SHGetKnownFolderPath](https://msdn.microsoft.com/en-us/library/windows/desktop/bb762188.aspx)
    /// API.
    pub fn path(self) -> PathBuf {
        known_path(&self.id()).expect("Folder::path")
    }
}

/// Represents a standard Windows [known folder](https://msdn.microsoft.com/en-us/library/windows/desktop/bb776911.aspx).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Folder {
    /// The [`FOLDERID_AccountPictures`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_AccountPictures) folder.
    AccountPictures,

    /// The [`FOLDERID_AddNewPrograms`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_AddNewPrograms) folder.
    AddNewPrograms,

    /// The [`FOLDERID_AdminTools`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_AdminTools) folder.
    AdminTools,

    /// The [`FOLDERID_AppDataDesktop`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_AppDataDesktop) folder.
    AppDataDesktop,

    /// The [`FOLDERID_AppDataDocuments`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_AppDataDocuments) folder.
    AppDataDocuments,

    /// The [`FOLDERID_AppDataFavorites`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_AppDataFavorites) folder.
    AppDataFavorites,

    /// The [`FOLDERID_AppDataProgramData`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_AppDataProgramData) folder.
    AppDataProgramData,

    /// The [`FOLDERID_ApplicationShortcuts`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ApplicationShortcuts) folder.
    ApplicationShortcuts,

    /// The [`FOLDERID_AppsFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_AppsFolder) folder.
    AppsFolder,

    /// The [`FOLDERID_AppUpdates`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_AppUpdates) folder.
    AppUpdates,

    /// The [`FOLDERID_CameraRoll`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_CameraRoll) folder.
    CameraRoll,

    /// The [`FOLDERID_CDBurning`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_CDBurning) folder.
    CDBurning,

    /// The [`FOLDERID_ChangeRemovePrograms`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ChangeRemovePrograms) folder.
    ChangeRemovePrograms,

    /// The [`FOLDERID_CommonAdminTools`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_CommonAdminTools) folder.
    CommonAdminTools,

    /// The [`FOLDERID_CommonOEMLinks`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_CommonOEMLinks) folder.
    CommonOEMLinks,

    /// The [`FOLDERID_CommonPrograms`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_CommonPrograms) folder.
    CommonPrograms,

    /// The [`FOLDERID_CommonStartMenu`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_CommonStartMenu) folder.
    CommonStartMenu,

    /// The [`FOLDERID_CommonStartup`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_CommonStartup) folder.
    CommonStartup,

    /// The [`FOLDERID_CommonTemplates`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_CommonTemplates) folder.
    CommonTemplates,

    /// The [`FOLDERID_ComputerFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ComputerFolder) folder.
    ComputerFolder,

    /// The [`FOLDERID_ConflictFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ConflictFolder) folder.
    ConflictFolder,

    /// The [`FOLDERID_ConnectionsFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ConnectionsFolder) folder.
    ConnectionsFolder,

    /// The [`FOLDERID_Contacts`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Contacts) folder.
    Contacts,

    /// The [`FOLDERID_ControlPanelFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ControlPanelFolder) folder.
    ControlPanelFolder,

    /// The [`FOLDERID_Cookies`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Cookies) folder.
    Cookies,

    /// The [`FOLDERID_Desktop`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Desktop) folder.
    Desktop,

    /// The [`FOLDERID_DeviceMetadataStore`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_DeviceMetadataStore) folder.
    DeviceMetadataStore,

    /// The [`FOLDERID_Documents`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Documents) folder.
    Documents,

    /// The [`FOLDERID_DocumentsLibrary`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_DocumentsLibrary) folder.
    DocumentsLibrary,

    /// The [`FOLDERID_Downloads`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Downloads) folder.
    Downloads,

    /// The [`FOLDERID_Favorites`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Favorites) folder.
    Favorites,

    /// The [`FOLDERID_Fonts`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Fonts) folder.
    Fonts,

    /// The [`FOLDERID_Games`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Games) folder.
    Games,

    /// The [`FOLDERID_GameTasks`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_GameTasks) folder.
    GameTasks,

    /// The [`FOLDERID_History`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_History) folder.
    History,

    /// The [`FOLDERID_HomeGroup`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_HomeGroup) folder.
    HomeGroup,

    /// The [`FOLDERID_HomeGroupCurrentUser`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_HomeGroupCurrentUser) folder.
    HomeGroupCurrentUser,

    /// The [`FOLDERID_ImplicitAppShortcuts`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ImplicitAppShortcuts) folder.
    ImplicitAppShortcuts,

    /// The [`FOLDERID_InternetCache`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_InternetCache) folder.
    InternetCache,

    /// The [`FOLDERID_InternetFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_InternetFolder) folder.
    InternetFolder,

    /// The [`FOLDERID_Libraries`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Libraries) folder.
    Libraries,

    /// The [`FOLDERID_Links`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Links) folder.
    Links,

    /// The [`FOLDERID_LocalAppData`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_LocalAppData) folder.
    LocalAppData,

    /// The [`FOLDERID_LocalAppDataLow`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_LocalAppDataLow) folder.
    LocalAppDataLow,

    /// The [`FOLDERID_LocalizedResourcesDir`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_LocalizedResourcesDir) folder.
    LocalizedResourcesDir,

    /// The [`FOLDERID_Music`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Music) folder.
    Music,

    /// The [`FOLDERID_MusicLibrary`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_MusicLibrary) folder.
    MusicLibrary,

    /// The [`FOLDERID_NetHood`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_NetHood) folder.
    NetHood,

    /// The [`FOLDERID_NetworkFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_NetworkFolder) folder.
    NetworkFolder,

    /// The [`FOLDERID_Objects3D`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Objects3D) folder.
    Objects3D,

    /// The [`FOLDERID_OriginalImages`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_OriginalImages) folder.
    OriginalImages,

    /// The [`FOLDERID_PhotoAlbums`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PhotoAlbums) folder.
    PhotoAlbums,

    /// The [`FOLDERID_PicturesLibrary`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PicturesLibrary) folder.
    PicturesLibrary,

    /// The [`FOLDERID_Pictures`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Pictures) folder.
    Pictures,

    /// The [`FOLDERID_Playlists`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Playlists) folder.
    Playlists,

    /// The [`FOLDERID_PrintersFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PrintersFolder) folder.
    PrintersFolder,

    /// The [`FOLDERID_PrintHood`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PrintHood) folder.
    PrintHood,

    /// The [`FOLDERID_Profile`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Profile) folder.
    Profile,

    /// The [`FOLDERID_ProgramData`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ProgramData) folder.
    ProgramData,

    /// The [`FOLDERID_ProgramFiles`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ProgramFiles) folder.
    ProgramFiles,

    /// The [`FOLDERID_ProgramFilesX64`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ProgramFilesX64) folder.
    ProgramFilesX64,

    /// The [`FOLDERID_ProgramFilesX86`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ProgramFilesX86) folder.
    ProgramFilesX86,

    /// The [`FOLDERID_ProgramFilesCommon`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ProgramFilesCommon) folder.
    ProgramFilesCommon,

    /// The [`FOLDERID_ProgramFilesCommonX64`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ProgramFilesCommonX64) folder.
    ProgramFilesCommonX64,

    /// The [`FOLDERID_ProgramFilesCommonX86`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ProgramFilesCommonX86) folder.
    ProgramFilesCommonX86,

    /// The [`FOLDERID_Programs`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Programs) folder.
    Programs,

    /// The [`FOLDERID_Public`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Public) folder.
    Public,

    /// The [`FOLDERID_PublicDesktop`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PublicDesktop) folder.
    PublicDesktop,

    /// The [`FOLDERID_PublicDocuments`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PublicDocuments) folder.
    PublicDocuments,

    /// The [`FOLDERID_PublicDownloads`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PublicDownloads) folder.
    PublicDownloads,

    /// The [`FOLDERID_PublicGameTasks`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PublicGameTasks) folder.
    PublicGameTasks,

    /// The [`FOLDERID_PublicLibraries`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PublicLibraries) folder.
    PublicLibraries,

    /// The [`FOLDERID_PublicMusic`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PublicMusic) folder.
    PublicMusic,

    /// The [`FOLDERID_PublicPictures`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PublicPictures) folder.
    PublicPictures,

    /// The [`FOLDERID_PublicRingtones`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PublicRingtones) folder.
    PublicRingtones,

    /// The [`FOLDERID_PublicUserTiles`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PublicUserTiles) folder.
    PublicUserTiles,

    /// The [`FOLDERID_PublicVideos`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_PublicVideos) folder.
    PublicVideos,

    /// The [`FOLDERID_QuickLaunch`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_QuickLaunch) folder.
    QuickLaunch,

    /// The [`FOLDERID_Recent`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Recent) folder.
    Recent,

    /// The [`FOLDERID_RecordedTVLibrary`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_RecordedTVLibrary) folder.
    RecordedTVLibrary,

    /// The [`FOLDERID_RecycleBinFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_RecycleBinFolder) folder.
    RecycleBinFolder,

    /// The [`FOLDERID_ResourceDir`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_ResourceDir) folder.
    ResourceDir,

    /// The [`FOLDERID_Ringtones`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Ringtones) folder.
    Ringtones,

    /// The [`FOLDERID_RoamingAppData`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_RoamingAppData) folder.
    RoamingAppData,

    /// The [`FOLDERID_RoamedTileImages`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_RoamedTileImages) folder.
    RoamedTileImages,

    /// The [`FOLDERID_RoamingTiles`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_RoamingTiles) folder.
    RoamingTiles,

    /// The [`FOLDERID_SampleMusic`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SampleMusic) folder.
    SampleMusic,

    /// The [`FOLDERID_SamplePictures`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SamplePictures) folder.
    SamplePictures,

    /// The [`FOLDERID_SamplePlaylists`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SamplePlaylists) folder.
    SamplePlaylists,

    /// The [`FOLDERID_SampleVideos`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SampleVideos) folder.
    SampleVideos,

    /// The [`FOLDERID_SavedGames`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SavedGames) folder.
    SavedGames,

    /// The [`FOLDERID_SavedPictures`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SavedPictures) folder.
    SavedPictures,

    /// The [`FOLDERID_SavedPicturesLibrary`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SavedPicturesLibrary) folder.
    SavedPicturesLibrary,

    /// The [`FOLDERID_SavedSearches`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SavedSearches) folder.
    SavedSearches,

    /// The [`FOLDERID_Screenshots`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Screenshots) folder.
    Screenshots,

    /// The [`FOLDERID_SEARCH_CSC`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SEARCH_CSC) folder.
    SearchCsc,

    /// The [`FOLDERID_SearchHistory`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SearchHistory) folder.
    SearchHistory,

    /// The [`FOLDERID_SearchHome`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SearchHome) folder.
    SearchHome,

    /// The [`FOLDERID_SEARCH_MAPI`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SEARCH_MAPI) folder.
    SearchMapi,

    /// The [`FOLDERID_SearchTemplates`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SearchTemplates) folder.
    SearchTemplates,

    /// The [`FOLDERID_SendTo`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SendTo) folder.
    SendTo,

    /// The [`FOLDERID_SidebarDefaultParts`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SidebarDefaultParts) folder.
    SidebarDefaultParts,

    /// The [`FOLDERID_SidebarParts`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SidebarParts) folder.
    SidebarParts,

    /// The [`FOLDERID_SkyDrive`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SkyDrive) folder.
    SkyDrive,

    /// The [`FOLDERID_SkyDriveCameraRoll`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SkyDriveCameraRoll) folder.
    SkyDriveCameraRoll,

    /// The [`FOLDERID_SkyDriveDocuments`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SkyDriveDocuments) folder.
    SkyDriveDocuments,

    /// The [`FOLDERID_SkyDrivePictures`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SkyDrivePictures) folder.
    SkyDrivePictures,

    /// The [`FOLDERID_StartMenu`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_StartMenu) folder.
    StartMenu,

    /// The [`FOLDERID_Startup`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Startup) folder.
    Startup,

    /// The [`FOLDERID_SyncManagerFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SyncManagerFolder) folder.
    SyncManagerFolder,

    /// The [`FOLDERID_SyncResultsFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SyncResultsFolder) folder.
    SyncResultsFolder,

    /// The [`FOLDERID_SyncSetupFolder`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SyncSetupFolder) folder.
    SyncSetupFolder,

    /// The [`FOLDERID_System`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_System) folder.
    System,

    /// The [`FOLDERID_SystemX86`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_SystemX86) folder.
    SystemX86,

    /// The [`FOLDERID_Templates`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Templates) folder.
    Templates,

    /// The [`FOLDERID_UserPinned`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_UserPinned) folder.
    UserPinned,

    /// The [`FOLDERID_UserProfiles`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_UserProfiles) folder.
    UserProfiles,

    /// The [`FOLDERID_UserProgramFiles`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_UserProgramFiles) folder.
    UserProgramFiles,

    /// The [`FOLDERID_UserProgramFilesCommon`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_UserProgramFilesCommon) folder.
    UserProgramFilesCommon,

    /// The [`FOLDERID_UsersFiles`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_UsersFiles) folder.
    UsersFiles,

    /// The [`FOLDERID_UsersLibraries`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_UsersLibraries) folder.
    UsersLibraries,

    /// The [`FOLDERID_Videos`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Videos) folder.
    Videos,

    /// The [`FOLDERID_VideosLibrary`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_VideosLibrary) folder.
    VideosLibrary,

    /// The [`FOLDERID_Windows`](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDER_ID_Windows) folder.
    Windows,
}

impl Folder {
    /// Returns the Windows GUID associated with this known folder.
    pub fn id(self) -> guid::GUID {
        use Folder::*;

        match self {AccountPictures => ACCOUNT_PICTURES,
            AddNewPrograms => ADD_NEW_PROGRAMS,
            AdminTools => ADMIN_TOOLS,
            AppDataDesktop => APP_DATA_DESKTOP,
            AppDataDocuments => APP_DATA_DOCUMENTS,
            AppDataFavorites => APP_DATA_FAVORITES,
            AppDataProgramData => APP_DATA_PROGRAM_DATA,
            ApplicationShortcuts => APPLICATION_SHORTCUTS,
            AppsFolder => APPS_FOLDER,
            AppUpdates => APP_UPDATES,
            CameraRoll => CAMERA_ROLL,
            CDBurning => CDBURNING,
            ChangeRemovePrograms => CHANGE_REMOVE_PROGRAMS,
            CommonAdminTools => COMMON_ADMIN_TOOLS,
            CommonOEMLinks => COMMON_OEMLINKS,
            CommonPrograms => COMMON_PROGRAMS,
            CommonStartMenu => COMMON_START_MENU,
            CommonStartup => COMMON_STARTUP,
            CommonTemplates => COMMON_TEMPLATES,
            ComputerFolder => COMPUTER_FOLDER,
            ConflictFolder => CONFLICT_FOLDER,
            ConnectionsFolder => CONNECTIONS_FOLDER,
            Contacts => CONTACTS,
            ControlPanelFolder => CONTROL_PANEL_FOLDER,
            Cookies => COOKIES,
            Desktop => DESKTOP,
            DeviceMetadataStore => DEVICE_METADATA_STORE,
            Documents => DOCUMENTS,
            DocumentsLibrary => DOCUMENTS_LIBRARY,
            Downloads => DOWNLOADS,
            Favorites => FAVORITES,
            Fonts => FONTS,
            Games => GAMES,
            GameTasks => GAME_TASKS,
            History => HISTORY,
            HomeGroup => HOME_GROUP,
            HomeGroupCurrentUser => HOME_GROUP_CURRENT_USER,
            ImplicitAppShortcuts => IMPLICIT_APP_SHORTCUTS,
            InternetCache => INTERNET_CACHE,
            InternetFolder => INTERNET_FOLDER,
            Libraries => LIBRARIES,
            Links => LINKS,
            LocalAppData => LOCAL_APP_DATA,
            LocalAppDataLow => LOCAL_APP_DATA_LOW,
            LocalizedResourcesDir => LOCALIZED_RESOURCES_DIR,
            Music => MUSIC,
            MusicLibrary => MUSIC_LIBRARY,
            NetHood => NET_HOOD,
            NetworkFolder => NETWORK_FOLDER,
            Objects3D => OBJECTS3D,
            OriginalImages => ORIGINAL_IMAGES,
            PhotoAlbums => PHOTO_ALBUMS,
            PicturesLibrary => PICTURES_LIBRARY,
            Pictures => PICTURES,
            Playlists => PLAYLISTS,
            PrintersFolder => PRINTERS_FOLDER,
            PrintHood => PRINT_HOOD,
            Profile => PROFILE,
            ProgramData => PROGRAM_DATA,
            ProgramFiles => PROGRAM_FILES,
            ProgramFilesX64 => PROGRAM_FILES_X64,
            ProgramFilesX86 => PROGRAM_FILES_X86,
            ProgramFilesCommon => PROGRAM_FILES_COMMON,
            ProgramFilesCommonX64 => PROGRAM_FILES_COMMON_X64,
            ProgramFilesCommonX86 => PROGRAM_FILES_COMMON_X86,
            Programs => PROGRAMS,
            Public => PUBLIC,
            PublicDesktop => PUBLIC_DESKTOP,
            PublicDocuments => PUBLIC_DOCUMENTS,
            PublicDownloads => PUBLIC_DOWNLOADS,
            PublicGameTasks => PUBLIC_GAME_TASKS,
            PublicLibraries => PUBLIC_LIBRARIES,
            PublicMusic => PUBLIC_MUSIC,
            PublicPictures => PUBLIC_PICTURES,
            PublicRingtones => PUBLIC_RINGTONES,
            PublicUserTiles => PUBLIC_USER_TILES,
            PublicVideos => PUBLIC_VIDEOS,
            QuickLaunch => QUICK_LAUNCH,
            Recent => RECENT,
            RecordedTVLibrary => RECORDED_TVLIBRARY,
            RecycleBinFolder => RECYCLE_BIN_FOLDER,
            ResourceDir => RESOURCE_DIR,
            Ringtones => RINGTONES,
            RoamingAppData => ROAMING_APP_DATA,
            RoamedTileImages => ROAMED_TILE_IMAGES,
            RoamingTiles => ROAMING_TILES,
            SampleMusic => SAMPLE_MUSIC,
            SamplePictures => SAMPLE_PICTURES,
            SamplePlaylists => SAMPLE_PLAYLISTS,
            SampleVideos => SAMPLE_VIDEOS,
            SavedGames => SAVED_GAMES,
            SavedPictures => SAVED_PICTURES,
            SavedPicturesLibrary => SAVED_PICTURES_LIBRARY,
            SavedSearches => SAVED_SEARCHES,
            Screenshots => SCREENSHOTS,
            SearchCsc => SEARCH_CSC,
            SearchHistory => SEARCH_HISTORY,
            SearchHome => SEARCH_HOME,
            SearchMapi => SEARCH_MAPI,
            SearchTemplates => SEARCH_TEMPLATES,
            SendTo => SEND_TO,
            SidebarDefaultParts => SIDEBAR_DEFAULT_PARTS,
            SidebarParts => SIDEBAR_PARTS,
            SkyDrive => SKY_DRIVE,
            SkyDriveCameraRoll => SKY_DRIVE_CAMERA_ROLL,
            SkyDriveDocuments => SKY_DRIVE_DOCUMENTS,
            SkyDrivePictures => SKY_DRIVE_PICTURES,
            StartMenu => START_MENU,
            Startup => STARTUP,
            SyncManagerFolder => SYNC_MANAGER_FOLDER,
            SyncResultsFolder => SYNC_RESULTS_FOLDER,
            SyncSetupFolder => SYNC_SETUP_FOLDER,
            System => SYSTEM,
            SystemX86 => SYSTEM_X86,
            Templates => TEMPLATES,
            UserPinned => USER_PINNED,
            UserProfiles => USER_PROFILES,
            UserProgramFiles => USER_PROGRAM_FILES,
            UserProgramFilesCommon => USER_PROGRAM_FILES_COMMON,
            UsersFiles => USERS_FILES,
            UsersLibraries => USERS_LIBRARIES,
            Videos => VIDEOS,
            VideosLibrary => VIDEOS_LIBRARY,
            Windows => WINDOWS,
        }
    }
}
