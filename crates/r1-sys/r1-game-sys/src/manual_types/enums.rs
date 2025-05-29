use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum gEGuild {
    /// gEGuild_None
    None = 0x00000000,
    /// gEGuild_Don
    Don = 0x00000001,
    /// gEGuild_Dig
    Dig = 0x00000002,
    /// gEGuild_Grd
    Grd = 0x00000003,
    /// gEGuild_Cit
    Cit = 0x00000004,
    /// gEGuild_Inq
    Inq = 0x00000005,
    /// gEGuild_Mag
    Mag = 0x00000006,
    /// gEGuild_Pir
    Pir = 0x00000007,
}

#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum gEInfoLocation {
    /// gEInfoLocation_Main     
    Main = 0x00000000,
    /// gEInfoLocation_Harbor   
    Harbor = 0x00000001,
    /// gEInfoLocation_Monastery
    Monastery = 0x00000002,
    /// gEInfoLocation_Don      
    Don = 0x00000003,
}

#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum gEHudPage {
    /// gEHudPage_None            
    None = 0x00000000,
    /// gEHudPage_Game            
    Game = 0x00000001,
    /// gEHudPage_Inventory       
    Inventory = 0x00000002,
    /// gEHudPage_Character       
    Character = 0x00000003,
    /// gEHudPage_Log             
    Log = 0x00000004,
    /// gEHudPage_Map             
    Map = 0x00000005,
    /// gEHudPage_CraftSelect     
    CraftSelect = 0x00000006,
    /// gEHudPage_ItemSelect      
    ItemSelect = 0x00000007,
    /// gEHudPage_Loot            
    Loot = 0x00000008,
    /// gEHudPage_Pickpocket      
    Pickpocket = 0x00000009,
    /// gEHudPage_Trade           
    Trade = 0x0000000A,
    /// gEHudPage_Dialog          
    Dialog = 0x0000000B,
    /// gEHudPage_Talk            
    Talk = 0x0000000C,
    /// gEHudPage_Menu_Back       
    MenuBack = 0x0000001A,
    /// gEHudPage_Menu_Main       
    MenuMain = 0x0000000D,
    /// gEHudPage_Menu_Game       
    MenuGame = 0x0000000E,
    /// gEHudPage_Menu_Load       
    MenuLoad = 0x0000000F,
    /// gEHudPage_Menu_Save       
    MenuSave = 0x00000010,
    /// gEHudPage_Menu_Achievement
    MenuAchievement = 0x00000011,
    /// gEHudPage_Menu_Options    
    MenuOptions = 0x00000012,
    /// gEHudPage_Menu_Video      
    MenuVideo = 0x00000013,
    /// gEHudPage_Menu_Audio      
    MenuAudio = 0x00000014,
    /// gEHudPage_Menu_Input      
    MenuInput = 0x00000015,
    /// gEHudPage_Menu_Settings   
    MenuSettings = 0x00000016,
    /// gEHudPage_Menu_System     
    MenuSystem = 0x00000017,
    /// gEHudPage_Menu_Credits    
    MenuCredits = 0x00000018,
    /// gEHudPage_Menu_Cheats     
    MenuCheats = 0x00000019,
    /// gEHudPage_Outro           
    Outro = 0x0000001B,
    /// gEHudPage_Loading         
    Loading = 0x0000001C,
}

#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum gESpecialEntity {
    /// gESpecialEntity_Player   
    Player = 0x00000000,
    /// gESpecialEntity_Focus    
    Focus = 0x00000001,
    /// gESpecialEntity_Interact
    Interact = 0x00000002,
    /// gESpecialEntity_Trader   
    Trader = 0x00000003,
    /// gESpecialEntity_DialogNPC
    DialogNPC = 0x00000004,
}

#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum gESkill {
    /// gESkill_None           
    None = 0xFFFFFFFF,
    /// gESkill_Atrib_HP       
    AttribHP = 0x00000000,
    /// gESkill_Atrib_MP       
    AttribMP = 0x00000001,
    /// gESkill_Stat_LV        
    StatLV = 0x00000002,
    /// gESkill_Stat_XP        
    StatXP = 0x00000003,
    /// gESkill_Stat_LP        
    StatLP = 0x00000004,
    /// gESkill_Stat_HP        
    StatHP = 0x00000005,
    /// gESkill_Stat_MP        
    StatMP = 0x00000006,
    /// gESkill_Stat_STR       
    StatSTR = 0x00000007,
    /// gESkill_Stat_DEX       
    StatDEX = 0x00000008,
    /// gESkill_Stat_INT       
    StatINT = 0x00000009,
    /// gESkill_Prot_Edge      
    ProtEdge = 0x0000000A,
    /// gESkill_Prot_Blunt     
    ProtBlunt = 0x0000000B,
    /// gESkill_Prot_Point     
    ProtPoint = 0x0000000C,
    /// gESkill_Prot_Fire      
    ProtFire = 0x0000000D,
    /// gESkill_Prot_Ice       
    Ice = 0x0000000E,
    /// gESkill_Prot_Magic     
    ProtMagic = 0x0000000F,
    /// gESkill_Combat_Sword   
    CombatSword = 0x00000010,
    /// gESkill_Combat_Axe     
    CombatAxe = 0x00000011,
    /// gESkill_Combat_Staff   
    CombatStaff = 0x00000012,
    /// gESkill_Combat_Bow     
    CombatBow = 0x00000013,
    /// gESkill_Combat_CrossBow
    CombatCrossBow = 0x00000014,
    /// gESkill_Magic_Circle   
    MagicCircle = 0x00000015,
    /// gESkill_Magic_Fireball
    MagicFireball = 0x00000016,
    /// gESkill_Magic_Frost    
    MagicFrost = 0x00000017,
    /// gESkill_Magic_Missile  
    MagicMissile = 0x00000018,
    /// gESkill_Misc_Scribe    
    MiscScribe = 0x00000020,
    /// gESkill_Misc_Alchemy   
    MiscAlchemy = 0x0000001F,
    /// gESkill_Misc_Smith     
    MiscSmith = 0x00000019,
    /// gESkill_Misc_Mining    
    MiscMining = 0x0000001A,
    /// gESkill_Misc_Sneak     
    MiscSneak = 0x0000001D,
    /// gESkill_Misc_Lockpick  
    MiscLockpick = 0x0000001B,
    /// gESkill_Misc_Pickpocket
    MiscPickpocket = 0x0000001C,
    /// gESkill_Misc_Acrobat   
    MiscAcrobat = 0x0000001E,
    /// gESkill_Misc_Trophy    
    MiscTrophy = 0x00000021,
}
