use super::loot_table::LootTable;
use super::materials::{self, Material};
use basin2_lib::AtomicSet;
use basin2_lib::{Registry, RegistryItem};
use std::fmt::Debug;
use std::sync::{atomic::AtomicU32, atomic::Ordering, Arc};
use std::any::Any;

mod data;
pub use data::*;
mod states;
pub use states::*;

pub trait BlockStateContainerImpl: Send + Sync + Debug + Any + 'static {}

#[derive(Debug)]
pub struct BlockT {
    pub registry_name: AtomicSet<String>,
    pub registry_id: AtomicU32,
    pub material: Material,
    // material_color ignored
    pub has_collision: bool,
    // sound type ignored
    pub light_emission: u8,
    pub explosion_resistance: f32,
    pub destroy_time: f32,
    pub is_ticking: bool,
    pub friction: f32,
    pub speed_factor: f32,
    pub jump_factor: f32,
    pub dynamic_shape: bool,
    pub drops: LootTable,
    pub can_occlude: bool,
    pub default_state: Option<Box<dyn BlockStateContainerImpl>>,
}

impl Default for BlockT {
    fn default() -> BlockT {
        BlockT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            material: materials::AIR.clone(),
            has_collision: true,
            light_emission: 0,
            explosion_resistance: 0.0,
            destroy_time: 0.0,
            is_ticking: false,
            friction: 0.6,
            speed_factor: 1.0,
            jump_factor: 1.0,
            drops: LootTable {},
            can_occlude: true,
            dynamic_shape: false,
            default_state: None,
        }
    }
}

impl RegistryItem for BlockT {
    fn registered(&self, key: &str, id: u32) {
        self.registry_name.try_set(key.to_string());
        self.registry_id.compare_and_swap(0, id, Ordering::Relaxed);
    }
}

pub fn construct_registry(registry: &mut Registry<BlockT>) {
    registry.insert("minecraft:air", AIR.clone());
    registry.insert("minecraft:stone", STONE.clone());
    registry.insert("minecraft:granite", GRANITE.clone());
    registry.insert("minecraft:polished_granite", POLISHED_GRANITE.clone());
    registry.insert("minecraft:diorite", DIORITE.clone());
    registry.insert("minecraft:polished_diorite", POLISHED_DIORITE.clone());
    registry.insert("minecraft:andesite", ANDESITE.clone());
    registry.insert("minecraft:polished_andesite", POLISHED_ANDESITE.clone());
    registry.insert("minecraft:grass_block", GRASS_BLOCK.clone());
    registry.insert("minecraft:dirt", DIRT.clone());
    registry.insert("minecraft:coarse_dirt", COARSE_DIRT.clone());
    registry.insert("minecraft:podzol", PODZOL.clone());
    registry.insert("minecraft:cobblestone", COBBLESTONE.clone());
    registry.insert("minecraft:oak_planks", OAK_PLANKS.clone());
    registry.insert("minecraft:spruce_planks", SPRUCE_PLANKS.clone());
    registry.insert("minecraft:birch_planks", BIRCH_PLANKS.clone());
    registry.insert("minecraft:jungle_planks", JUNGLE_PLANKS.clone());
    registry.insert("minecraft:acacia_planks", ACACIA_PLANKS.clone());
    registry.insert("minecraft:dark_oak_planks", DARK_OAK_PLANKS.clone());
    registry.insert("minecraft:oak_sapling", OAK_SAPLING.clone());
    registry.insert("minecraft:spruce_sapling", SPRUCE_SAPLING.clone());
    registry.insert("minecraft:birch_sapling", BIRCH_SAPLING.clone());
    registry.insert("minecraft:jungle_sapling", JUNGLE_SAPLING.clone());
    registry.insert("minecraft:acacia_sapling", ACACIA_SAPLING.clone());
    registry.insert("minecraft:dark_oak_sapling", DARK_OAK_SAPLING.clone());
    registry.insert("minecraft:bedrock", BEDROCK.clone());
    registry.insert("minecraft:water", WATER.clone());
    registry.insert("minecraft:lava", LAVA.clone());
    registry.insert("minecraft:sand", SAND.clone());
    registry.insert("minecraft:red_sand", RED_SAND.clone());
    registry.insert("minecraft:gravel", GRAVEL.clone());
    registry.insert("minecraft:gold_ore", GOLD_ORE.clone());
    registry.insert("minecraft:iron_ore", IRON_ORE.clone());
    registry.insert("minecraft:coal_ore", COAL_ORE.clone());
    registry.insert("minecraft:oak_log", OAK_LOG.clone());
    registry.insert("minecraft:spruce_log", SPRUCE_LOG.clone());
    registry.insert("minecraft:birch_log", BIRCH_LOG.clone());
    registry.insert("minecraft:jungle_log", JUNGLE_LOG.clone());
    registry.insert("minecraft:acacia_log", ACACIA_LOG.clone());
    registry.insert("minecraft:dark_oak_log", DARK_OAK_LOG.clone());
    registry.insert("minecraft:stripped_spruce_log", STRIPPED_SPRUCE_LOG.clone());
    registry.insert("minecraft:stripped_birch_log", STRIPPED_BIRCH_LOG.clone());
    registry.insert("minecraft:stripped_jungle_log", STRIPPED_JUNGLE_LOG.clone());
    registry.insert("minecraft:stripped_acacia_log", STRIPPED_ACACIA_LOG.clone());
    registry.insert(
        "minecraft:stripped_dark_oak_log",
        STRIPPED_DARK_OAK_LOG.clone(),
    );
    registry.insert("minecraft:stripped_oak_log", STRIPPED_OAK_LOG.clone());
    registry.insert("minecraft:oak_wood", OAK_WOOD.clone());
    registry.insert("minecraft:spruce_wood", SPRUCE_WOOD.clone());
    registry.insert("minecraft:birch_wood", BIRCH_WOOD.clone());
    registry.insert("minecraft:jungle_wood", JUNGLE_WOOD.clone());
    registry.insert("minecraft:acacia_wood", ACACIA_WOOD.clone());
    registry.insert("minecraft:dark_oak_wood", DARK_OAK_WOOD.clone());
    registry.insert("minecraft:stripped_oak_wood", STRIPPED_OAK_WOOD.clone());
    registry.insert(
        "minecraft:stripped_spruce_wood",
        STRIPPED_SPRUCE_WOOD.clone(),
    );
    registry.insert("minecraft:stripped_birch_wood", STRIPPED_BIRCH_WOOD.clone());
    registry.insert(
        "minecraft:stripped_jungle_wood",
        STRIPPED_JUNGLE_WOOD.clone(),
    );
    registry.insert(
        "minecraft:stripped_acacia_wood",
        STRIPPED_ACACIA_WOOD.clone(),
    );
    registry.insert(
        "minecraft:stripped_dark_oak_wood",
        STRIPPED_DARK_OAK_WOOD.clone(),
    );
    registry.insert("minecraft:oak_leaves", OAK_LEAVES.clone());
    registry.insert("minecraft:spruce_leaves", SPRUCE_LEAVES.clone());
    registry.insert("minecraft:birch_leaves", BIRCH_LEAVES.clone());
    registry.insert("minecraft:jungle_leaves", JUNGLE_LEAVES.clone());
    registry.insert("minecraft:acacia_leaves", ACACIA_LEAVES.clone());
    registry.insert("minecraft:dark_oak_leaves", DARK_OAK_LEAVES.clone());
    registry.insert("minecraft:sponge", SPONGE.clone());
    registry.insert("minecraft:wet_sponge", WET_SPONGE.clone());
    registry.insert("minecraft:glass", GLASS.clone());
    registry.insert("minecraft:lapis_ore", LAPIS_ORE.clone());
    registry.insert("minecraft:lapis_block", LAPIS_BLOCK.clone());
    registry.insert("minecraft:dispenser", DISPENSER.clone());
    registry.insert("minecraft:sandstone", SANDSTONE.clone());
    registry.insert("minecraft:chiseled_sandstone", CHISELED_SANDSTONE.clone());
    registry.insert("minecraft:cut_sandstone", CUT_SANDSTONE.clone());
    registry.insert("minecraft:note_block", NOTE_BLOCK.clone());
    registry.insert("minecraft:white_bed", WHITE_BED.clone());
    registry.insert("minecraft:orange_bed", ORANGE_BED.clone());
    registry.insert("minecraft:magenta_bed", MAGENTA_BED.clone());
    registry.insert("minecraft:light_blue_bed", LIGHT_BLUE_BED.clone());
    registry.insert("minecraft:yellow_bed", YELLOW_BED.clone());
    registry.insert("minecraft:lime_bed", LIME_BED.clone());
    registry.insert("minecraft:pink_bed", PINK_BED.clone());
    registry.insert("minecraft:gray_bed", GRAY_BED.clone());
    registry.insert("minecraft:light_gray_bed", LIGHT_GRAY_BED.clone());
    registry.insert("minecraft:cyan_bed", CYAN_BED.clone());
    registry.insert("minecraft:purple_bed", PURPLE_BED.clone());
    registry.insert("minecraft:blue_bed", BLUE_BED.clone());
    registry.insert("minecraft:brown_bed", BROWN_BED.clone());
    registry.insert("minecraft:green_bed", GREEN_BED.clone());
    registry.insert("minecraft:red_bed", RED_BED.clone());
    registry.insert("minecraft:black_bed", BLACK_BED.clone());
    registry.insert("minecraft:powered_rail", POWERED_RAIL.clone());
    registry.insert("minecraft:detector_rail", DETECTOR_RAIL.clone());
    registry.insert("minecraft:sticky_piston", STICKY_PISTON.clone());
    registry.insert("minecraft:cobweb", COBWEB.clone());
    registry.insert("minecraft:grass", GRASS.clone());
    registry.insert("minecraft:fern", FERN.clone());
    registry.insert("minecraft:dead_bush", DEAD_BUSH.clone());
    registry.insert("minecraft:seagrass", SEAGRASS.clone());
    registry.insert("minecraft:tall_seagrass", TALL_SEAGRASS.clone());
    registry.insert("minecraft:piston", PISTON.clone());
    registry.insert("minecraft:piston_head", PISTON_HEAD.clone());
    registry.insert("minecraft:white_wool", WHITE_WOOL.clone());
    registry.insert("minecraft:orange_wool", ORANGE_WOOL.clone());
    registry.insert("minecraft:magenta_wool", MAGENTA_WOOL.clone());
    registry.insert("minecraft:light_blue_wool", LIGHT_BLUE_WOOL.clone());
    registry.insert("minecraft:yellow_wool", YELLOW_WOOL.clone());
    registry.insert("minecraft:lime_wool", LIME_WOOL.clone());
    registry.insert("minecraft:pink_wool", PINK_WOOL.clone());
    registry.insert("minecraft:gray_wool", GRAY_WOOL.clone());
    registry.insert("minecraft:light_gray_wool", LIGHT_GRAY_WOOL.clone());
    registry.insert("minecraft:cyan_wool", CYAN_WOOL.clone());
    registry.insert("minecraft:purple_wool", PURPLE_WOOL.clone());
    registry.insert("minecraft:blue_wool", BLUE_WOOL.clone());
    registry.insert("minecraft:brown_wool", BROWN_WOOL.clone());
    registry.insert("minecraft:green_wool", GREEN_WOOL.clone());
    registry.insert("minecraft:red_wool", RED_WOOL.clone());
    registry.insert("minecraft:black_wool", BLACK_WOOL.clone());
    registry.insert("minecraft:moving_piston", MOVING_PISTON.clone());
    registry.insert("minecraft:dandelion", DANDELION.clone());
    registry.insert("minecraft:poppy", POPPY.clone());
    registry.insert("minecraft:blue_orchid", BLUE_ORCHID.clone());
    registry.insert("minecraft:allium", ALLIUM.clone());
    registry.insert("minecraft:azure_bluet", AZURE_BLUET.clone());
    registry.insert("minecraft:red_tulip", RED_TULIP.clone());
    registry.insert("minecraft:orange_tulip", ORANGE_TULIP.clone());
    registry.insert("minecraft:white_tulip", WHITE_TULIP.clone());
    registry.insert("minecraft:pink_tulip", PINK_TULIP.clone());
    registry.insert("minecraft:oxeye_daisy", OXEYE_DAISY.clone());
    registry.insert("minecraft:cornflower", CORNFLOWER.clone());
    registry.insert("minecraft:wither_rose", WITHER_ROSE.clone());
    registry.insert("minecraft:lily_of_the_valley", LILY_OF_THE_VALLEY.clone());
    registry.insert("minecraft:brown_mushroom", BROWN_MUSHROOM.clone());
    registry.insert("minecraft:red_mushroom", RED_MUSHROOM.clone());
    registry.insert("minecraft:gold_block", GOLD_BLOCK.clone());
    registry.insert("minecraft:iron_block", IRON_BLOCK.clone());
    registry.insert("minecraft:bricks", BRICKS.clone());
    registry.insert("minecraft:tnt", TNT.clone());
    registry.insert("minecraft:bookshelf", BOOKSHELF.clone());
    registry.insert("minecraft:mossy_cobblestone", MOSSY_COBBLESTONE.clone());
    registry.insert("minecraft:obsidian", OBSIDIAN.clone());
    registry.insert("minecraft:torch", TORCH.clone());
    registry.insert("minecraft:wall_torch", WALL_TORCH.clone());
    registry.insert("minecraft:fire", FIRE.clone());
    registry.insert("minecraft:spawner", SPAWNER.clone());
    registry.insert("minecraft:oak_stairs", OAK_STAIRS.clone());
    registry.insert("minecraft:chest", CHEST.clone());
    registry.insert("minecraft:redstone_wire", REDSTONE_WIRE.clone());
    registry.insert("minecraft:diamond_ore", DIAMOND_ORE.clone());
    registry.insert("minecraft:diamond_block", DIAMOND_BLOCK.clone());
    registry.insert("minecraft:crafting_table", CRAFTING_TABLE.clone());
    registry.insert("minecraft:wheat", WHEAT.clone());
    registry.insert("minecraft:farmland", FARMLAND.clone());
    registry.insert("minecraft:furnace", FURNACE.clone());
    registry.insert("minecraft:oak_sign", OAK_SIGN.clone());
    registry.insert("minecraft:spruce_sign", SPRUCE_SIGN.clone());
    registry.insert("minecraft:birch_sign", BIRCH_SIGN.clone());
    registry.insert("minecraft:acacia_sign", ACACIA_SIGN.clone());
    registry.insert("minecraft:jungle_sign", JUNGLE_SIGN.clone());
    registry.insert("minecraft:dark_oak_sign", DARK_OAK_SIGN.clone());
    registry.insert("minecraft:oak_door", OAK_DOOR.clone());
    registry.insert("minecraft:ladder", LADDER.clone());
    registry.insert("minecraft:rail", RAIL.clone());
    registry.insert("minecraft:cobblestone_stairs", COBBLESTONE_STAIRS.clone());
    registry.insert("minecraft:oak_wall_sign", OAK_WALL_SIGN.clone());
    registry.insert("minecraft:spruce_wall_sign", SPRUCE_WALL_SIGN.clone());
    registry.insert("minecraft:birch_wall_sign", BIRCH_WALL_SIGN.clone());
    registry.insert("minecraft:acacia_wall_sign", ACACIA_WALL_SIGN.clone());
    registry.insert("minecraft:jungle_wall_sign", JUNGLE_WALL_SIGN.clone());
    registry.insert("minecraft:dark_oak_wall_sign", DARK_OAK_WALL_SIGN.clone());
    registry.insert("minecraft:lever", LEVER.clone());
    registry.insert(
        "minecraft:stone_pressure_plate",
        STONE_PRESSURE_PLATE.clone(),
    );
    registry.insert("minecraft:iron_door", IRON_DOOR.clone());
    registry.insert("minecraft:oak_pressure_plate", OAK_PRESSURE_PLATE.clone());
    registry.insert(
        "minecraft:spruce_pressure_plate",
        SPRUCE_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        "minecraft:birch_pressure_plate",
        BIRCH_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        "minecraft:jungle_pressure_plate",
        JUNGLE_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        "minecraft:acacia_pressure_plate",
        ACACIA_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        "minecraft:dark_oak_pressure_plate",
        DARK_OAK_PRESSURE_PLATE.clone(),
    );
    registry.insert("minecraft:redstone_ore", REDSTONE_ORE.clone());
    registry.insert("minecraft:redstone_torch", REDSTONE_TORCH.clone());
    registry.insert("minecraft:redstone_wall_torch", REDSTONE_WALL_TORCH.clone());
    registry.insert("minecraft:stone_button", STONE_BUTTON.clone());
    registry.insert("minecraft:snow", SNOW.clone());
    registry.insert("minecraft:ice", ICE.clone());
    registry.insert("minecraft:snow_block", SNOW_BLOCK.clone());
    registry.insert("minecraft:cactus", CACTUS.clone());
    registry.insert("minecraft:clay", CLAY.clone());
    registry.insert("minecraft:sugar_cane", SUGAR_CANE.clone());
    registry.insert("minecraft:jukebox", JUKEBOX.clone());
    registry.insert("minecraft:oak_fence", OAK_FENCE.clone());
    registry.insert("minecraft:pumpkin", PUMPKIN.clone());
    registry.insert("minecraft:netherrack", NETHERRACK.clone());
    registry.insert("minecraft:soul_sand", SOUL_SAND.clone());
    registry.insert("minecraft:glowstone", GLOWSTONE.clone());
    registry.insert("minecraft:nether_portal", NETHER_PORTAL.clone());
    registry.insert("minecraft:carved_pumpkin", CARVED_PUMPKIN.clone());
    registry.insert("minecraft:jack_o_lantern", JACK_O_LANTERN.clone());
    registry.insert("minecraft:cake", CAKE.clone());
    registry.insert("minecraft:repeater", REPEATER.clone());
    registry.insert("minecraft:white_stained_glass", WHITE_STAINED_GLASS.clone());
    registry.insert(
        "minecraft:orange_stained_glass",
        ORANGE_STAINED_GLASS.clone(),
    );
    registry.insert(
        "minecraft:magenta_stained_glass",
        MAGENTA_STAINED_GLASS.clone(),
    );
    registry.insert(
        "minecraft:light_blue_stained_glass",
        LIGHT_BLUE_STAINED_GLASS.clone(),
    );
    registry.insert(
        "minecraft:yellow_stained_glass",
        YELLOW_STAINED_GLASS.clone(),
    );
    registry.insert("minecraft:lime_stained_glass", LIME_STAINED_GLASS.clone());
    registry.insert("minecraft:pink_stained_glass", PINK_STAINED_GLASS.clone());
    registry.insert("minecraft:gray_stained_glass", GRAY_STAINED_GLASS.clone());
    registry.insert(
        "minecraft:light_gray_stained_glass",
        LIGHT_GRAY_STAINED_GLASS.clone(),
    );
    registry.insert("minecraft:cyan_stained_glass", CYAN_STAINED_GLASS.clone());
    registry.insert(
        "minecraft:purple_stained_glass",
        PURPLE_STAINED_GLASS.clone(),
    );
    registry.insert("minecraft:blue_stained_glass", BLUE_STAINED_GLASS.clone());
    registry.insert("minecraft:brown_stained_glass", BROWN_STAINED_GLASS.clone());
    registry.insert("minecraft:green_stained_glass", GREEN_STAINED_GLASS.clone());
    registry.insert("minecraft:red_stained_glass", RED_STAINED_GLASS.clone());
    registry.insert("minecraft:black_stained_glass", BLACK_STAINED_GLASS.clone());
    registry.insert("minecraft:oak_trapdoor", OAK_TRAPDOOR.clone());
    registry.insert("minecraft:spruce_trapdoor", SPRUCE_TRAPDOOR.clone());
    registry.insert("minecraft:birch_trapdoor", BIRCH_TRAPDOOR.clone());
    registry.insert("minecraft:jungle_trapdoor", JUNGLE_TRAPDOOR.clone());
    registry.insert("minecraft:acacia_trapdoor", ACACIA_TRAPDOOR.clone());
    registry.insert("minecraft:dark_oak_trapdoor", DARK_OAK_TRAPDOOR.clone());
    registry.insert("minecraft:stone_bricks", STONE_BRICKS.clone());
    registry.insert("minecraft:mossy_stone_bricks", MOSSY_STONE_BRICKS.clone());
    registry.insert(
        "minecraft:cracked_stone_bricks",
        CRACKED_STONE_BRICKS.clone(),
    );
    registry.insert(
        "minecraft:chiseled_stone_bricks",
        CHISELED_STONE_BRICKS.clone(),
    );
    registry.insert("minecraft:infested_stone", INFESTED_STONE.clone());
    registry.insert(
        "minecraft:infested_cobblestone",
        INFESTED_COBBLESTONE.clone(),
    );
    registry.insert(
        "minecraft:infested_stone_bricks",
        INFESTED_STONE_BRICKS.clone(),
    );
    registry.insert(
        "minecraft:infested_mossy_stone_bricks",
        INFESTED_MOSSY_STONE_BRICKS.clone(),
    );
    registry.insert(
        "minecraft:infested_cracked_stone_bricks",
        INFESTED_CRACKED_STONE_BRICKS.clone(),
    );
    registry.insert(
        "minecraft:infested_chiseled_stone_bricks",
        INFESTED_CHISELED_STONE_BRICKS.clone(),
    );
    registry.insert(
        "minecraft:brown_mushroom_block",
        BROWN_MUSHROOM_BLOCK.clone(),
    );
    registry.insert("minecraft:red_mushroom_block", RED_MUSHROOM_BLOCK.clone());
    registry.insert("minecraft:mushroom_stem", MUSHROOM_STEM.clone());
    registry.insert("minecraft:iron_bars", IRON_BARS.clone());
    registry.insert("minecraft:glass_pane", GLASS_PANE.clone());
    registry.insert("minecraft:melon", MELON.clone());
    registry.insert(
        "minecraft:attached_pumpkin_stem",
        ATTACHED_PUMPKIN_STEM.clone(),
    );
    registry.insert("minecraft:attached_melon_stem", ATTACHED_MELON_STEM.clone());
    registry.insert("minecraft:pumpkin_stem", PUMPKIN_STEM.clone());
    registry.insert("minecraft:melon_stem", MELON_STEM.clone());
    registry.insert("minecraft:vine", VINE.clone());
    registry.insert("minecraft:oak_fence_gate", OAK_FENCE_GATE.clone());
    registry.insert("minecraft:brick_stairs", BRICK_STAIRS.clone());
    registry.insert("minecraft:stone_brick_stairs", STONE_BRICK_STAIRS.clone());
    registry.insert("minecraft:mycelium", MYCELIUM.clone());
    registry.insert("minecraft:lily_pad", LILY_PAD.clone());
    registry.insert("minecraft:nether_bricks", NETHER_BRICKS.clone());
    registry.insert("minecraft:nether_brick_fence", NETHER_BRICK_FENCE.clone());
    registry.insert("minecraft:nether_brick_stairs", NETHER_BRICK_STAIRS.clone());
    registry.insert("minecraft:nether_wart", NETHER_WART.clone());
    registry.insert("minecraft:enchanting_table", ENCHANTING_TABLE.clone());
    registry.insert("minecraft:brewing_stand", BREWING_STAND.clone());
    registry.insert("minecraft:cauldron", CAULDRON.clone());
    registry.insert("minecraft:end_portal", END_PORTAL.clone());
    registry.insert("minecraft:end_portal_frame", END_PORTAL_FRAME.clone());
    registry.insert("minecraft:end_stone", END_STONE.clone());
    registry.insert("minecraft:dragon_egg", DRAGON_EGG.clone());
    registry.insert("minecraft:redstone_lamp", REDSTONE_LAMP.clone());
    registry.insert("minecraft:cocoa", COCOA.clone());
    registry.insert("minecraft:sandstone_stairs", SANDSTONE_STAIRS.clone());
    registry.insert("minecraft:emerald_ore", EMERALD_ORE.clone());
    registry.insert("minecraft:ender_chest", ENDER_CHEST.clone());
    registry.insert("minecraft:tripwire_hook", TRIPWIRE_HOOK.clone());
    registry.insert("minecraft:tripwire", TRIPWIRE.clone());
    registry.insert("minecraft:emerald_block", EMERALD_BLOCK.clone());
    registry.insert("minecraft:spruce_stairs", SPRUCE_STAIRS.clone());
    registry.insert("minecraft:birch_stairs", BIRCH_STAIRS.clone());
    registry.insert("minecraft:jungle_stairs", JUNGLE_STAIRS.clone());
    registry.insert("minecraft:command_block", COMMAND_BLOCK.clone());
    registry.insert("minecraft:beacon", BEACON.clone());
    registry.insert("minecraft:cobblestone_wall", COBBLESTONE_WALL.clone());
    registry.insert(
        "minecraft:mossy_cobblestone_wall",
        MOSSY_COBBLESTONE_WALL.clone(),
    );
    registry.insert("minecraft:flower_pot", FLOWER_POT.clone());
    registry.insert("minecraft:potted_oak_sapling", POTTED_OAK_SAPLING.clone());
    registry.insert(
        "minecraft:potted_spruce_sapling",
        POTTED_SPRUCE_SAPLING.clone(),
    );
    registry.insert(
        "minecraft:potted_birch_sapling",
        POTTED_BIRCH_SAPLING.clone(),
    );
    registry.insert(
        "minecraft:potted_jungle_sapling",
        POTTED_JUNGLE_SAPLING.clone(),
    );
    registry.insert(
        "minecraft:potted_acacia_sapling",
        POTTED_ACACIA_SAPLING.clone(),
    );
    registry.insert(
        "minecraft:potted_dark_oak_sapling",
        POTTED_DARK_OAK_SAPLING.clone(),
    );
    registry.insert("minecraft:potted_fern", POTTED_FERN.clone());
    registry.insert("minecraft:potted_dandelion", POTTED_DANDELION.clone());
    registry.insert("minecraft:potted_poppy", POTTED_POPPY.clone());
    registry.insert("minecraft:potted_blue_orchid", POTTED_BLUE_ORCHID.clone());
    registry.insert("minecraft:potted_allium", POTTED_ALLIUM.clone());
    registry.insert("minecraft:potted_azure_bluet", POTTED_AZURE_BLUET.clone());
    registry.insert("minecraft:potted_red_tulip", POTTED_RED_TULIP.clone());
    registry.insert("minecraft:potted_orange_tulip", POTTED_ORANGE_TULIP.clone());
    registry.insert("minecraft:potted_white_tulip", POTTED_WHITE_TULIP.clone());
    registry.insert("minecraft:potted_pink_tulip", POTTED_PINK_TULIP.clone());
    registry.insert("minecraft:potted_oxeye_daisy", POTTED_OXEYE_DAISY.clone());
    registry.insert("minecraft:potted_cornflower", POTTED_CORNFLOWER.clone());
    registry.insert(
        "minecraft:potted_lily_of_the_valley",
        POTTED_LILY_OF_THE_VALLEY.clone(),
    );
    registry.insert("minecraft:potted_wither_rose", POTTED_WITHER_ROSE.clone());
    registry.insert("minecraft:potted_red_mushroom", POTTED_RED_MUSHROOM.clone());
    registry.insert(
        "minecraft:potted_brown_mushroom",
        POTTED_BROWN_MUSHROOM.clone(),
    );
    registry.insert("minecraft:potted_dead_bush", POTTED_DEAD_BUSH.clone());
    registry.insert("minecraft:potted_cactus", POTTED_CACTUS.clone());
    registry.insert("minecraft:carrots", CARROTS.clone());
    registry.insert("minecraft:potatoes", POTATOES.clone());
    registry.insert("minecraft:oak_button", OAK_BUTTON.clone());
    registry.insert("minecraft:spruce_button", SPRUCE_BUTTON.clone());
    registry.insert("minecraft:birch_button", BIRCH_BUTTON.clone());
    registry.insert("minecraft:jungle_button", JUNGLE_BUTTON.clone());
    registry.insert("minecraft:acacia_button", ACACIA_BUTTON.clone());
    registry.insert("minecraft:dark_oak_button", DARK_OAK_BUTTON.clone());
    registry.insert("minecraft:skeleton_skull", SKELETON_SKULL.clone());
    registry.insert("minecraft:skeleton_wall_skull", SKELETON_WALL_SKULL.clone());
    registry.insert(
        "minecraft:wither_skeleton_skull",
        WITHER_SKELETON_SKULL.clone(),
    );
    registry.insert(
        "minecraft:wither_skeleton_wall_skull",
        WITHER_SKELETON_WALL_SKULL.clone(),
    );
    registry.insert("minecraft:zombie_head", ZOMBIE_HEAD.clone());
    registry.insert("minecraft:zombie_wall_head", ZOMBIE_WALL_HEAD.clone());
    registry.insert("minecraft:player_head", PLAYER_HEAD.clone());
    registry.insert("minecraft:player_wall_head", PLAYER_WALL_HEAD.clone());
    registry.insert("minecraft:creeper_head", CREEPER_HEAD.clone());
    registry.insert("minecraft:creeper_wall_head", CREEPER_WALL_HEAD.clone());
    registry.insert("minecraft:dragon_head", DRAGON_HEAD.clone());
    registry.insert("minecraft:dragon_wall_head", DRAGON_WALL_HEAD.clone());
    registry.insert("minecraft:anvil", ANVIL.clone());
    registry.insert("minecraft:chipped_anvil", CHIPPED_ANVIL.clone());
    registry.insert("minecraft:damaged_anvil", DAMAGED_ANVIL.clone());
    registry.insert("minecraft:trapped_chest", TRAPPED_CHEST.clone());
    registry.insert(
        "minecraft:light_weighted_pressure_plate",
        LIGHT_WEIGHTED_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        "minecraft:heavy_weighted_pressure_plate",
        HEAVY_WEIGHTED_PRESSURE_PLATE.clone(),
    );
    registry.insert("minecraft:comparator", COMPARATOR.clone());
    registry.insert("minecraft:daylight_detector", DAYLIGHT_DETECTOR.clone());
    registry.insert("minecraft:redstone_block", REDSTONE_BLOCK.clone());
    registry.insert("minecraft:nether_quartz_ore", NETHER_QUARTZ_ORE.clone());
    registry.insert("minecraft:hopper", HOPPER.clone());
    registry.insert("minecraft:quartz_block", QUARTZ_BLOCK.clone());
    registry.insert(
        "minecraft:chiseled_quartz_block",
        CHISELED_QUARTZ_BLOCK.clone(),
    );
    registry.insert("minecraft:quartz_pillar", QUARTZ_PILLAR.clone());
    registry.insert("minecraft:quartz_stairs", QUARTZ_STAIRS.clone());
    registry.insert("minecraft:activator_rail", ACTIVATOR_RAIL.clone());
    registry.insert("minecraft:dropper", DROPPER.clone());
    registry.insert("minecraft:white_terracotta", WHITE_TERRACOTTA.clone());
    registry.insert("minecraft:orange_terracotta", ORANGE_TERRACOTTA.clone());
    registry.insert("minecraft:magenta_terracotta", MAGENTA_TERRACOTTA.clone());
    registry.insert(
        "minecraft:light_blue_terracotta",
        LIGHT_BLUE_TERRACOTTA.clone(),
    );
    registry.insert("minecraft:yellow_terracotta", YELLOW_TERRACOTTA.clone());
    registry.insert("minecraft:lime_terracotta", LIME_TERRACOTTA.clone());
    registry.insert("minecraft:pink_terracotta", PINK_TERRACOTTA.clone());
    registry.insert("minecraft:gray_terracotta", GRAY_TERRACOTTA.clone());
    registry.insert(
        "minecraft:light_gray_terracotta",
        LIGHT_GRAY_TERRACOTTA.clone(),
    );
    registry.insert("minecraft:cyan_terracotta", CYAN_TERRACOTTA.clone());
    registry.insert("minecraft:purple_terracotta", PURPLE_TERRACOTTA.clone());
    registry.insert("minecraft:blue_terracotta", BLUE_TERRACOTTA.clone());
    registry.insert("minecraft:brown_terracotta", BROWN_TERRACOTTA.clone());
    registry.insert("minecraft:green_terracotta", GREEN_TERRACOTTA.clone());
    registry.insert("minecraft:red_terracotta", RED_TERRACOTTA.clone());
    registry.insert("minecraft:black_terracotta", BLACK_TERRACOTTA.clone());
    registry.insert(
        "minecraft:white_stained_glass_pane",
        WHITE_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:orange_stained_glass_pane",
        ORANGE_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:magenta_stained_glass_pane",
        MAGENTA_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:light_blue_stained_glass_pane",
        LIGHT_BLUE_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:yellow_stained_glass_pane",
        YELLOW_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:lime_stained_glass_pane",
        LIME_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:pink_stained_glass_pane",
        PINK_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:gray_stained_glass_pane",
        GRAY_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:light_gray_stained_glass_pane",
        LIGHT_GRAY_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:cyan_stained_glass_pane",
        CYAN_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:purple_stained_glass_pane",
        PURPLE_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:blue_stained_glass_pane",
        BLUE_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:brown_stained_glass_pane",
        BROWN_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:green_stained_glass_pane",
        GREEN_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:red_stained_glass_pane",
        RED_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        "minecraft:black_stained_glass_pane",
        BLACK_STAINED_GLASS_PANE.clone(),
    );
    registry.insert("minecraft:acacia_stairs", ACACIA_STAIRS.clone());
    registry.insert("minecraft:dark_oak_stairs", DARK_OAK_STAIRS.clone());
    registry.insert("minecraft:slime_block", SLIME_BLOCK.clone());
    registry.insert("minecraft:barrier", BARRIER.clone());
    registry.insert("minecraft:iron_trapdoor", IRON_TRAPDOOR.clone());
    registry.insert("minecraft:prismarine", PRISMARINE.clone());
    registry.insert("minecraft:prismarine_bricks", PRISMARINE_BRICKS.clone());
    registry.insert("minecraft:dark_prismarine", DARK_PRISMARINE.clone());
    registry.insert("minecraft:prismarine_stairs", PRISMARINE_STAIRS.clone());
    registry.insert(
        "minecraft:prismarine_brick_stairs",
        PRISMARINE_BRICK_STAIRS.clone(),
    );
    registry.insert(
        "minecraft:dark_prismarine_stairs",
        DARK_PRISMARINE_STAIRS.clone(),
    );
    registry.insert("minecraft:prismarine_slab", PRISMARINE_SLAB.clone());
    registry.insert(
        "minecraft:prismarine_brick_slab",
        PRISMARINE_BRICK_SLAB.clone(),
    );
    registry.insert(
        "minecraft:dark_prismarine_slab",
        DARK_PRISMARINE_SLAB.clone(),
    );
    registry.insert("minecraft:sea_lantern", SEA_LANTERN.clone());
    registry.insert("minecraft:hay_block", HAY_BLOCK.clone());
    registry.insert("minecraft:white_carpet", WHITE_CARPET.clone());
    registry.insert("minecraft:orange_carpet", ORANGE_CARPET.clone());
    registry.insert("minecraft:magenta_carpet", MAGENTA_CARPET.clone());
    registry.insert("minecraft:light_blue_carpet", LIGHT_BLUE_CARPET.clone());
    registry.insert("minecraft:yellow_carpet", YELLOW_CARPET.clone());
    registry.insert("minecraft:lime_carpet", LIME_CARPET.clone());
    registry.insert("minecraft:pink_carpet", PINK_CARPET.clone());
    registry.insert("minecraft:gray_carpet", GRAY_CARPET.clone());
    registry.insert("minecraft:light_gray_carpet", LIGHT_GRAY_CARPET.clone());
    registry.insert("minecraft:cyan_carpet", CYAN_CARPET.clone());
    registry.insert("minecraft:purple_carpet", PURPLE_CARPET.clone());
    registry.insert("minecraft:blue_carpet", BLUE_CARPET.clone());
    registry.insert("minecraft:brown_carpet", BROWN_CARPET.clone());
    registry.insert("minecraft:green_carpet", GREEN_CARPET.clone());
    registry.insert("minecraft:red_carpet", RED_CARPET.clone());
    registry.insert("minecraft:black_carpet", BLACK_CARPET.clone());
    registry.insert("minecraft:terracotta", TERRACOTTA.clone());
    registry.insert("minecraft:coal_block", COAL_BLOCK.clone());
    registry.insert("minecraft:packed_ice", PACKED_ICE.clone());
    registry.insert("minecraft:sunflower", SUNFLOWER.clone());
    registry.insert("minecraft:lilac", LILAC.clone());
    registry.insert("minecraft:rose_bush", ROSE_BUSH.clone());
    registry.insert("minecraft:peony", PEONY.clone());
    registry.insert("minecraft:tall_grass", TALL_GRASS.clone());
    registry.insert("minecraft:large_fern", LARGE_FERN.clone());
    registry.insert("minecraft:white_banner", WHITE_BANNER.clone());
    registry.insert("minecraft:orange_banner", ORANGE_BANNER.clone());
    registry.insert("minecraft:magenta_banner", MAGENTA_BANNER.clone());
    registry.insert("minecraft:light_blue_banner", LIGHT_BLUE_BANNER.clone());
    registry.insert("minecraft:yellow_banner", YELLOW_BANNER.clone());
    registry.insert("minecraft:lime_banner", LIME_BANNER.clone());
    registry.insert("minecraft:pink_banner", PINK_BANNER.clone());
    registry.insert("minecraft:gray_banner", GRAY_BANNER.clone());
    registry.insert("minecraft:light_gray_banner", LIGHT_GRAY_BANNER.clone());
    registry.insert("minecraft:cyan_banner", CYAN_BANNER.clone());
    registry.insert("minecraft:purple_banner", PURPLE_BANNER.clone());
    registry.insert("minecraft:blue_banner", BLUE_BANNER.clone());
    registry.insert("minecraft:brown_banner", BROWN_BANNER.clone());
    registry.insert("minecraft:green_banner", GREEN_BANNER.clone());
    registry.insert("minecraft:red_banner", RED_BANNER.clone());
    registry.insert("minecraft:black_banner", BLACK_BANNER.clone());
    registry.insert("minecraft:white_wall_banner", WHITE_WALL_BANNER.clone());
    registry.insert("minecraft:orange_wall_banner", ORANGE_WALL_BANNER.clone());
    registry.insert("minecraft:magenta_wall_banner", MAGENTA_WALL_BANNER.clone());
    registry.insert(
        "minecraft:light_blue_wall_banner",
        LIGHT_BLUE_WALL_BANNER.clone(),
    );
    registry.insert("minecraft:yellow_wall_banner", YELLOW_WALL_BANNER.clone());
    registry.insert("minecraft:lime_wall_banner", LIME_WALL_BANNER.clone());
    registry.insert("minecraft:pink_wall_banner", PINK_WALL_BANNER.clone());
    registry.insert("minecraft:gray_wall_banner", GRAY_WALL_BANNER.clone());
    registry.insert(
        "minecraft:light_gray_wall_banner",
        LIGHT_GRAY_WALL_BANNER.clone(),
    );
    registry.insert("minecraft:cyan_wall_banner", CYAN_WALL_BANNER.clone());
    registry.insert("minecraft:purple_wall_banner", PURPLE_WALL_BANNER.clone());
    registry.insert("minecraft:blue_wall_banner", BLUE_WALL_BANNER.clone());
    registry.insert("minecraft:brown_wall_banner", BROWN_WALL_BANNER.clone());
    registry.insert("minecraft:green_wall_banner", GREEN_WALL_BANNER.clone());
    registry.insert("minecraft:red_wall_banner", RED_WALL_BANNER.clone());
    registry.insert("minecraft:black_wall_banner", BLACK_WALL_BANNER.clone());
    registry.insert("minecraft:red_sandstone", RED_SANDSTONE.clone());
    registry.insert(
        "minecraft:chiseled_red_sandstone",
        CHISELED_RED_SANDSTONE.clone(),
    );
    registry.insert("minecraft:cut_red_sandstone", CUT_RED_SANDSTONE.clone());
    registry.insert(
        "minecraft:red_sandstone_stairs",
        RED_SANDSTONE_STAIRS.clone(),
    );
    registry.insert("minecraft:oak_slab", OAK_SLAB.clone());
    registry.insert("minecraft:spruce_slab", SPRUCE_SLAB.clone());
    registry.insert("minecraft:birch_slab", BIRCH_SLAB.clone());
    registry.insert("minecraft:jungle_slab", JUNGLE_SLAB.clone());
    registry.insert("minecraft:acacia_slab", ACACIA_SLAB.clone());
    registry.insert("minecraft:dark_oak_slab", DARK_OAK_SLAB.clone());
    registry.insert("minecraft:stone_slab", STONE_SLAB.clone());
    registry.insert("minecraft:smooth_stone_slab", SMOOTH_STONE_SLAB.clone());
    registry.insert("minecraft:sandstone_slab", SANDSTONE_SLAB.clone());
    registry.insert("minecraft:cut_sandstone_slab", CUT_SANDSTONE_SLAB.clone());
    registry.insert("minecraft:petrified_oak_slab", PETRIFIED_OAK_SLAB.clone());
    registry.insert("minecraft:cobblestone_slab", COBBLESTONE_SLAB.clone());
    registry.insert("minecraft:brick_slab", BRICK_SLAB.clone());
    registry.insert("minecraft:stone_brick_slab", STONE_BRICK_SLAB.clone());
    registry.insert("minecraft:nether_brick_slab", NETHER_BRICK_SLAB.clone());
    registry.insert("minecraft:quartz_slab", QUARTZ_SLAB.clone());
    registry.insert("minecraft:red_sandstone_slab", RED_SANDSTONE_SLAB.clone());
    registry.insert(
        "minecraft:cut_red_sandstone_slab",
        CUT_RED_SANDSTONE_SLAB.clone(),
    );
    registry.insert("minecraft:purpur_slab", PURPUR_SLAB.clone());
    registry.insert("minecraft:smooth_stone", SMOOTH_STONE.clone());
    registry.insert("minecraft:smooth_sandstone", SMOOTH_SANDSTONE.clone());
    registry.insert("minecraft:smooth_quartz", SMOOTH_QUARTZ.clone());
    registry.insert(
        "minecraft:smooth_red_sandstone",
        SMOOTH_RED_SANDSTONE.clone(),
    );
    registry.insert("minecraft:spruce_fence_gate", SPRUCE_FENCE_GATE.clone());
    registry.insert("minecraft:birch_fence_gate", BIRCH_FENCE_GATE.clone());
    registry.insert("minecraft:jungle_fence_gate", JUNGLE_FENCE_GATE.clone());
    registry.insert("minecraft:acacia_fence_gate", ACACIA_FENCE_GATE.clone());
    registry.insert("minecraft:dark_oak_fence_gate", DARK_OAK_FENCE_GATE.clone());
    registry.insert("minecraft:spruce_fence", SPRUCE_FENCE.clone());
    registry.insert("minecraft:birch_fence", BIRCH_FENCE.clone());
    registry.insert("minecraft:jungle_fence", JUNGLE_FENCE.clone());
    registry.insert("minecraft:acacia_fence", ACACIA_FENCE.clone());
    registry.insert("minecraft:dark_oak_fence", DARK_OAK_FENCE.clone());
    registry.insert("minecraft:spruce_door", SPRUCE_DOOR.clone());
    registry.insert("minecraft:birch_door", BIRCH_DOOR.clone());
    registry.insert("minecraft:jungle_door", JUNGLE_DOOR.clone());
    registry.insert("minecraft:acacia_door", ACACIA_DOOR.clone());
    registry.insert("minecraft:dark_oak_door", DARK_OAK_DOOR.clone());
    registry.insert("minecraft:end_rod", END_ROD.clone());
    registry.insert("minecraft:chorus_plant", CHORUS_PLANT.clone());
    registry.insert("minecraft:chorus_flower", CHORUS_FLOWER.clone());
    registry.insert("minecraft:purpur_block", PURPUR_BLOCK.clone());
    registry.insert("minecraft:purpur_pillar", PURPUR_PILLAR.clone());
    registry.insert("minecraft:purpur_stairs", PURPUR_STAIRS.clone());
    registry.insert("minecraft:end_stone_bricks", END_STONE_BRICKS.clone());
    registry.insert("minecraft:beetroots", BEETROOTS.clone());
    registry.insert("minecraft:grass_path", GRASS_PATH.clone());
    registry.insert("minecraft:end_gateway", END_GATEWAY.clone());
    registry.insert(
        "minecraft:repeating_command_block",
        REPEATING_COMMAND_BLOCK.clone(),
    );
    registry.insert("minecraft:chain_command_block", CHAIN_COMMAND_BLOCK.clone());
    registry.insert("minecraft:frosted_ice", FROSTED_ICE.clone());
    registry.insert("minecraft:magma_block", MAGMA_BLOCK.clone());
    registry.insert("minecraft:nether_wart_block", NETHER_WART_BLOCK.clone());
    registry.insert("minecraft:red_nether_bricks", RED_NETHER_BRICKS.clone());
    registry.insert("minecraft:bone_block", BONE_BLOCK.clone());
    registry.insert("minecraft:structure_void", STRUCTURE_VOID.clone());
    registry.insert("minecraft:observer", OBSERVER.clone());
    registry.insert("minecraft:shulker_box", SHULKER_BOX.clone());
    registry.insert("minecraft:white_shulker_box", WHITE_SHULKER_BOX.clone());
    registry.insert("minecraft:orange_shulker_box", ORANGE_SHULKER_BOX.clone());
    registry.insert("minecraft:magenta_shulker_box", MAGENTA_SHULKER_BOX.clone());
    registry.insert(
        "minecraft:light_blue_shulker_box",
        LIGHT_BLUE_SHULKER_BOX.clone(),
    );
    registry.insert("minecraft:yellow_shulker_box", YELLOW_SHULKER_BOX.clone());
    registry.insert("minecraft:lime_shulker_box", LIME_SHULKER_BOX.clone());
    registry.insert("minecraft:pink_shulker_box", PINK_SHULKER_BOX.clone());
    registry.insert("minecraft:gray_shulker_box", GRAY_SHULKER_BOX.clone());
    registry.insert(
        "minecraft:light_gray_shulker_box",
        LIGHT_GRAY_SHULKER_BOX.clone(),
    );
    registry.insert("minecraft:cyan_shulker_box", CYAN_SHULKER_BOX.clone());
    registry.insert("minecraft:purple_shulker_box", PURPLE_SHULKER_BOX.clone());
    registry.insert("minecraft:blue_shulker_box", BLUE_SHULKER_BOX.clone());
    registry.insert("minecraft:brown_shulker_box", BROWN_SHULKER_BOX.clone());
    registry.insert("minecraft:green_shulker_box", GREEN_SHULKER_BOX.clone());
    registry.insert("minecraft:red_shulker_box", RED_SHULKER_BOX.clone());
    registry.insert("minecraft:black_shulker_box", BLACK_SHULKER_BOX.clone());
    registry.insert(
        "minecraft:white_glazed_terracotta",
        WHITE_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:orange_glazed_terracotta",
        ORANGE_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:magenta_glazed_terracotta",
        MAGENTA_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:light_blue_glazed_terracotta",
        LIGHT_BLUE_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:yellow_glazed_terracotta",
        YELLOW_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:lime_glazed_terracotta",
        LIME_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:pink_glazed_terracotta",
        PINK_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:gray_glazed_terracotta",
        GRAY_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:light_gray_glazed_terracotta",
        LIGHT_GRAY_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:cyan_glazed_terracotta",
        CYAN_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:purple_glazed_terracotta",
        PURPLE_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:blue_glazed_terracotta",
        BLUE_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:brown_glazed_terracotta",
        BROWN_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:green_glazed_terracotta",
        GREEN_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:red_glazed_terracotta",
        RED_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        "minecraft:black_glazed_terracotta",
        BLACK_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert("minecraft:white_concrete", WHITE_CONCRETE.clone());
    registry.insert("minecraft:orange_concrete", ORANGE_CONCRETE.clone());
    registry.insert("minecraft:magenta_concrete", MAGENTA_CONCRETE.clone());
    registry.insert("minecraft:light_blue_concrete", LIGHT_BLUE_CONCRETE.clone());
    registry.insert("minecraft:yellow_concrete", YELLOW_CONCRETE.clone());
    registry.insert("minecraft:lime_concrete", LIME_CONCRETE.clone());
    registry.insert("minecraft:pink_concrete", PINK_CONCRETE.clone());
    registry.insert("minecraft:gray_concrete", GRAY_CONCRETE.clone());
    registry.insert("minecraft:light_gray_concrete", LIGHT_GRAY_CONCRETE.clone());
    registry.insert("minecraft:cyan_concrete", CYAN_CONCRETE.clone());
    registry.insert("minecraft:purple_concrete", PURPLE_CONCRETE.clone());
    registry.insert("minecraft:blue_concrete", BLUE_CONCRETE.clone());
    registry.insert("minecraft:brown_concrete", BROWN_CONCRETE.clone());
    registry.insert("minecraft:green_concrete", GREEN_CONCRETE.clone());
    registry.insert("minecraft:red_concrete", RED_CONCRETE.clone());
    registry.insert("minecraft:black_concrete", BLACK_CONCRETE.clone());
    registry.insert(
        "minecraft:white_concrete_powder",
        WHITE_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:orange_concrete_powder",
        ORANGE_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:magenta_concrete_powder",
        MAGENTA_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:light_blue_concrete_powder",
        LIGHT_BLUE_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:yellow_concrete_powder",
        YELLOW_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:lime_concrete_powder",
        LIME_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:pink_concrete_powder",
        PINK_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:gray_concrete_powder",
        GRAY_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:light_gray_concrete_powder",
        LIGHT_GRAY_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:cyan_concrete_powder",
        CYAN_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:purple_concrete_powder",
        PURPLE_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:blue_concrete_powder",
        BLUE_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:brown_concrete_powder",
        BROWN_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        "minecraft:green_concrete_powder",
        GREEN_CONCRETE_POWDER.clone(),
    );
    registry.insert("minecraft:red_concrete_powder", RED_CONCRETE_POWDER.clone());
    registry.insert(
        "minecraft:black_concrete_powder",
        BLACK_CONCRETE_POWDER.clone(),
    );
    registry.insert("minecraft:kelp", KELP.clone());
    registry.insert("minecraft:kelp_plant", KELP_PLANT.clone());
    registry.insert("minecraft:dried_kelp_block", DRIED_KELP_BLOCK.clone());
    registry.insert("minecraft:turtle_egg", TURTLE_EGG.clone());
    registry.insert(
        "minecraft:dead_tube_coral_block",
        DEAD_TUBE_CORAL_BLOCK.clone(),
    );
    registry.insert(
        "minecraft:dead_brain_coral_block",
        DEAD_BRAIN_CORAL_BLOCK.clone(),
    );
    registry.insert(
        "minecraft:dead_bubble_coral_block",
        DEAD_BUBBLE_CORAL_BLOCK.clone(),
    );
    registry.insert(
        "minecraft:dead_fire_coral_block",
        DEAD_FIRE_CORAL_BLOCK.clone(),
    );
    registry.insert(
        "minecraft:dead_horn_coral_block",
        DEAD_HORN_CORAL_BLOCK.clone(),
    );
    registry.insert("minecraft:tube_coral_block", TUBE_CORAL_BLOCK.clone());
    registry.insert("minecraft:brain_coral_block", BRAIN_CORAL_BLOCK.clone());
    registry.insert("minecraft:bubble_coral_block", BUBBLE_CORAL_BLOCK.clone());
    registry.insert("minecraft:fire_coral_block", FIRE_CORAL_BLOCK.clone());
    registry.insert("minecraft:horn_coral_block", HORN_CORAL_BLOCK.clone());
    registry.insert("minecraft:dead_tube_coral", DEAD_TUBE_CORAL.clone());
    registry.insert("minecraft:dead_brain_coral", DEAD_BRAIN_CORAL.clone());
    registry.insert("minecraft:dead_bubble_coral", DEAD_BUBBLE_CORAL.clone());
    registry.insert("minecraft:dead_fire_coral", DEAD_FIRE_CORAL.clone());
    registry.insert("minecraft:dead_horn_coral", DEAD_HORN_CORAL.clone());
    registry.insert("minecraft:tube_coral", TUBE_CORAL.clone());
    registry.insert("minecraft:brain_coral", BRAIN_CORAL.clone());
    registry.insert("minecraft:bubble_coral", BUBBLE_CORAL.clone());
    registry.insert("minecraft:fire_coral", FIRE_CORAL.clone());
    registry.insert("minecraft:horn_coral", HORN_CORAL.clone());
    registry.insert("minecraft:dead_tube_coral_fan", DEAD_TUBE_CORAL_FAN.clone());
    registry.insert(
        "minecraft:dead_brain_coral_fan",
        DEAD_BRAIN_CORAL_FAN.clone(),
    );
    registry.insert(
        "minecraft:dead_bubble_coral_fan",
        DEAD_BUBBLE_CORAL_FAN.clone(),
    );
    registry.insert("minecraft:dead_fire_coral_fan", DEAD_FIRE_CORAL_FAN.clone());
    registry.insert("minecraft:dead_horn_coral_fan", DEAD_HORN_CORAL_FAN.clone());
    registry.insert("minecraft:tube_coral_fan", TUBE_CORAL_FAN.clone());
    registry.insert("minecraft:brain_coral_fan", BRAIN_CORAL_FAN.clone());
    registry.insert("minecraft:bubble_coral_fan", BUBBLE_CORAL_FAN.clone());
    registry.insert("minecraft:fire_coral_fan", FIRE_CORAL_FAN.clone());
    registry.insert("minecraft:horn_coral_fan", HORN_CORAL_FAN.clone());
    registry.insert(
        "minecraft:dead_tube_coral_wall_fan",
        DEAD_TUBE_CORAL_WALL_FAN.clone(),
    );
    registry.insert(
        "minecraft:dead_brain_coral_wall_fan",
        DEAD_BRAIN_CORAL_WALL_FAN.clone(),
    );
    registry.insert(
        "minecraft:dead_bubble_coral_wall_fan",
        DEAD_BUBBLE_CORAL_WALL_FAN.clone(),
    );
    registry.insert(
        "minecraft:dead_fire_coral_wall_fan",
        DEAD_FIRE_CORAL_WALL_FAN.clone(),
    );
    registry.insert(
        "minecraft:dead_horn_coral_wall_fan",
        DEAD_HORN_CORAL_WALL_FAN.clone(),
    );
    registry.insert("minecraft:tube_coral_wall_fan", TUBE_CORAL_WALL_FAN.clone());
    registry.insert(
        "minecraft:brain_coral_wall_fan",
        BRAIN_CORAL_WALL_FAN.clone(),
    );
    registry.insert(
        "minecraft:bubble_coral_wall_fan",
        BUBBLE_CORAL_WALL_FAN.clone(),
    );
    registry.insert("minecraft:fire_coral_wall_fan", FIRE_CORAL_WALL_FAN.clone());
    registry.insert("minecraft:horn_coral_wall_fan", HORN_CORAL_WALL_FAN.clone());
    registry.insert("minecraft:sea_pickle", SEA_PICKLE.clone());
    registry.insert("minecraft:blue_ice", BLUE_ICE.clone());
    registry.insert("minecraft:conduit", CONDUIT.clone());
    registry.insert("minecraft:bamboo_sapling", BAMBOO_SAPLING.clone());
    registry.insert("minecraft:bamboo", BAMBOO.clone());
    registry.insert("minecraft:potted_bamboo", POTTED_BAMBOO.clone());
    registry.insert("minecraft:void_air", VOID_AIR.clone());
    registry.insert("minecraft:cave_air", CAVE_AIR.clone());
    registry.insert("minecraft:bubble_column", BUBBLE_COLUMN.clone());
    registry.insert(
        "minecraft:polished_granite_stairs",
        POLISHED_GRANITE_STAIRS.clone(),
    );
    registry.insert(
        "minecraft:smooth_red_sandstone_stairs",
        SMOOTH_RED_SANDSTONE_STAIRS.clone(),
    );
    registry.insert(
        "minecraft:mossy_stone_brick_stairs",
        MOSSY_STONE_BRICK_STAIRS.clone(),
    );
    registry.insert(
        "minecraft:polished_diorite_stairs",
        POLISHED_DIORITE_STAIRS.clone(),
    );
    registry.insert(
        "minecraft:mossy_cobblestone_stairs",
        MOSSY_COBBLESTONE_STAIRS.clone(),
    );
    registry.insert(
        "minecraft:end_stone_brick_stairs",
        END_STONE_BRICK_STAIRS.clone(),
    );
    registry.insert("minecraft:stone_stairs", STONE_STAIRS.clone());
    registry.insert(
        "minecraft:smooth_sandstone_stairs",
        SMOOTH_SANDSTONE_STAIRS.clone(),
    );
    registry.insert(
        "minecraft:smooth_quartz_stairs",
        SMOOTH_QUARTZ_STAIRS.clone(),
    );
    registry.insert("minecraft:granite_stairs", GRANITE_STAIRS.clone());
    registry.insert("minecraft:andesite_stairs", ANDESITE_STAIRS.clone());
    registry.insert(
        "minecraft:red_nether_brick_stairs",
        RED_NETHER_BRICK_STAIRS.clone(),
    );
    registry.insert(
        "minecraft:polished_andesite_stairs",
        POLISHED_ANDESITE_STAIRS.clone(),
    );
    registry.insert("minecraft:diorite_stairs", DIORITE_STAIRS.clone());
    registry.insert(
        "minecraft:polished_granite_slab",
        POLISHED_GRANITE_SLAB.clone(),
    );
    registry.insert(
        "minecraft:smooth_red_sandstone_slab",
        SMOOTH_RED_SANDSTONE_SLAB.clone(),
    );
    registry.insert(
        "minecraft:mossy_stone_brick_slab",
        MOSSY_STONE_BRICK_SLAB.clone(),
    );
    registry.insert(
        "minecraft:polished_diorite_slab",
        POLISHED_DIORITE_SLAB.clone(),
    );
    registry.insert(
        "minecraft:mossy_cobblestone_slab",
        MOSSY_COBBLESTONE_SLAB.clone(),
    );
    registry.insert(
        "minecraft:end_stone_brick_slab",
        END_STONE_BRICK_SLAB.clone(),
    );
    registry.insert(
        "minecraft:smooth_sandstone_slab",
        SMOOTH_SANDSTONE_SLAB.clone(),
    );
    registry.insert("minecraft:smooth_quartz_slab", SMOOTH_QUARTZ_SLAB.clone());
    registry.insert("minecraft:granite_slab", GRANITE_SLAB.clone());
    registry.insert("minecraft:andesite_slab", ANDESITE_SLAB.clone());
    registry.insert(
        "minecraft:red_nether_brick_slab",
        RED_NETHER_BRICK_SLAB.clone(),
    );
    registry.insert(
        "minecraft:polished_andesite_slab",
        POLISHED_ANDESITE_SLAB.clone(),
    );
    registry.insert("minecraft:diorite_slab", DIORITE_SLAB.clone());
    registry.insert("minecraft:brick_wall", BRICK_WALL.clone());
    registry.insert("minecraft:prismarine_wall", PRISMARINE_WALL.clone());
    registry.insert("minecraft:red_sandstone_wall", RED_SANDSTONE_WALL.clone());
    registry.insert(
        "minecraft:mossy_stone_brick_wall",
        MOSSY_STONE_BRICK_WALL.clone(),
    );
    registry.insert("minecraft:granite_wall", GRANITE_WALL.clone());
    registry.insert("minecraft:stone_brick_wall", STONE_BRICK_WALL.clone());
    registry.insert("minecraft:nether_brick_wall", NETHER_BRICK_WALL.clone());
    registry.insert("minecraft:andesite_wall", ANDESITE_WALL.clone());
    registry.insert(
        "minecraft:red_nether_brick_wall",
        RED_NETHER_BRICK_WALL.clone(),
    );
    registry.insert("minecraft:sandstone_wall", SANDSTONE_WALL.clone());
    registry.insert(
        "minecraft:end_stone_brick_wall",
        END_STONE_BRICK_WALL.clone(),
    );
    registry.insert("minecraft:diorite_wall", DIORITE_WALL.clone());
    registry.insert("minecraft:scaffolding", SCAFFOLDING.clone());
    registry.insert("minecraft:loom", LOOM.clone());
    registry.insert("minecraft:barrel", BARREL.clone());
    registry.insert("minecraft:smoker", SMOKER.clone());
    registry.insert("minecraft:blast_furnace", BLAST_FURNACE.clone());
    registry.insert("minecraft:cartography_table", CARTOGRAPHY_TABLE.clone());
    registry.insert("minecraft:fletching_table", FLETCHING_TABLE.clone());
    registry.insert("minecraft:grindstone", GRINDSTONE.clone());
    registry.insert("minecraft:lectern", LECTERN.clone());
    registry.insert("minecraft:smithing_table", SMITHING_TABLE.clone());
    registry.insert("minecraft:stonecutter", STONECUTTER.clone());
    registry.insert("minecraft:bell", BELL.clone());
    registry.insert("minecraft:lantern", LANTERN.clone());
    registry.insert("minecraft:campfire", CAMPFIRE.clone());
    registry.insert("minecraft:sweet_berry_bush", SWEET_BERRY_BUSH.clone());
    registry.insert("minecraft:structure_block", STRUCTURE_BLOCK.clone());
    registry.insert("minecraft:jigsaw", JIGSAW.clone());
    registry.insert("minecraft:composter", COMPOSTER.clone());
    registry.insert("minecraft:bee_nest", BEE_NEST.clone());
    registry.insert("minecraft:beehive", BEEHIVE.clone());
    registry.insert("minecraft:honey_block", HONEY_BLOCK.clone());
    registry.insert("minecraft:honeycomb_block", HONEYCOMB_BLOCK.clone());
}

pub type Block = Arc<BlockT>;
