use super::*;
use crate::blocks;
use crate::mob_effects;
use basin2_lib::AtomicSet;
use std::sync::Arc;

lazy_static! {
    pub static ref AIR: Item = { Arc::new(ItemT::from(blocks::AIR.clone())) };
}
lazy_static! {
    pub static ref STONE: Item = { Arc::new(ItemT::from(blocks::STONE.clone())) };
}
lazy_static! {
    pub static ref GRANITE: Item = { Arc::new(ItemT::from(blocks::GRANITE.clone())) };
}
lazy_static! {
    pub static ref POLISHED_GRANITE: Item =
        { Arc::new(ItemT::from(blocks::POLISHED_GRANITE.clone())) };
}
lazy_static! {
    pub static ref DIORITE: Item = { Arc::new(ItemT::from(blocks::DIORITE.clone())) };
}
lazy_static! {
    pub static ref POLISHED_DIORITE: Item =
        { Arc::new(ItemT::from(blocks::POLISHED_DIORITE.clone())) };
}
lazy_static! {
    pub static ref ANDESITE: Item = { Arc::new(ItemT::from(blocks::ANDESITE.clone())) };
}
lazy_static! {
    pub static ref POLISHED_ANDESITE: Item =
        { Arc::new(ItemT::from(blocks::POLISHED_ANDESITE.clone())) };
}
lazy_static! {
    pub static ref GRASS_BLOCK: Item = { Arc::new(ItemT::from(blocks::GRASS_BLOCK.clone())) };
}
lazy_static! {
    pub static ref DIRT: Item = { Arc::new(ItemT::from(blocks::DIRT.clone())) };
}
lazy_static! {
    pub static ref COARSE_DIRT: Item = { Arc::new(ItemT::from(blocks::COARSE_DIRT.clone())) };
}
lazy_static! {
    pub static ref PODZOL: Item = { Arc::new(ItemT::from(blocks::PODZOL.clone())) };
}
lazy_static! {
    pub static ref COBBLESTONE: Item = { Arc::new(ItemT::from(blocks::COBBLESTONE.clone())) };
}
lazy_static! {
    pub static ref OAK_PLANKS: Item = { Arc::new(ItemT::from(blocks::OAK_PLANKS.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_PLANKS: Item = { Arc::new(ItemT::from(blocks::SPRUCE_PLANKS.clone())) };
}
lazy_static! {
    pub static ref BIRCH_PLANKS: Item = { Arc::new(ItemT::from(blocks::BIRCH_PLANKS.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_PLANKS: Item = { Arc::new(ItemT::from(blocks::JUNGLE_PLANKS.clone())) };
}
lazy_static! {
    pub static ref ACACIA_PLANKS: Item = { Arc::new(ItemT::from(blocks::ACACIA_PLANKS.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_PLANKS: Item =
        { Arc::new(ItemT::from(blocks::DARK_OAK_PLANKS.clone())) };
}
lazy_static! {
    pub static ref OAK_SAPLING: Item = { Arc::new(ItemT::from(blocks::OAK_SAPLING.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_SAPLING: Item = { Arc::new(ItemT::from(blocks::SPRUCE_SAPLING.clone())) };
}
lazy_static! {
    pub static ref BIRCH_SAPLING: Item = { Arc::new(ItemT::from(blocks::BIRCH_SAPLING.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_SAPLING: Item = { Arc::new(ItemT::from(blocks::JUNGLE_SAPLING.clone())) };
}
lazy_static! {
    pub static ref ACACIA_SAPLING: Item = { Arc::new(ItemT::from(blocks::ACACIA_SAPLING.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_SAPLING: Item =
        { Arc::new(ItemT::from(blocks::DARK_OAK_SAPLING.clone())) };
}
lazy_static! {
    pub static ref BEDROCK: Item = { Arc::new(ItemT::from(blocks::BEDROCK.clone())) };
}
lazy_static! {
    pub static ref SAND: Item = { Arc::new(ItemT::from(blocks::SAND.clone())) };
}
lazy_static! {
    pub static ref RED_SAND: Item = { Arc::new(ItemT::from(blocks::RED_SAND.clone())) };
}
lazy_static! {
    pub static ref GRAVEL: Item = { Arc::new(ItemT::from(blocks::GRAVEL.clone())) };
}
lazy_static! {
    pub static ref GOLD_ORE: Item = { Arc::new(ItemT::from(blocks::GOLD_ORE.clone())) };
}
lazy_static! {
    pub static ref IRON_ORE: Item = { Arc::new(ItemT::from(blocks::IRON_ORE.clone())) };
}
lazy_static! {
    pub static ref COAL_ORE: Item = { Arc::new(ItemT::from(blocks::COAL_ORE.clone())) };
}
lazy_static! {
    pub static ref OAK_LOG: Item = { Arc::new(ItemT::from(blocks::OAK_LOG.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_LOG: Item = { Arc::new(ItemT::from(blocks::SPRUCE_LOG.clone())) };
}
lazy_static! {
    pub static ref BIRCH_LOG: Item = { Arc::new(ItemT::from(blocks::BIRCH_LOG.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_LOG: Item = { Arc::new(ItemT::from(blocks::JUNGLE_LOG.clone())) };
}
lazy_static! {
    pub static ref ACACIA_LOG: Item = { Arc::new(ItemT::from(blocks::ACACIA_LOG.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_LOG: Item = { Arc::new(ItemT::from(blocks::DARK_OAK_LOG.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_OAK_LOG: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_OAK_LOG.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_SPRUCE_LOG: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_SPRUCE_LOG.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_BIRCH_LOG: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_BIRCH_LOG.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_JUNGLE_LOG: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_JUNGLE_LOG.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_ACACIA_LOG: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_ACACIA_LOG.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_DARK_OAK_LOG: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_DARK_OAK_LOG.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_OAK_WOOD: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_OAK_WOOD.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_SPRUCE_WOOD: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_SPRUCE_WOOD.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_BIRCH_WOOD: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_BIRCH_WOOD.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_JUNGLE_WOOD: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_JUNGLE_WOOD.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_ACACIA_WOOD: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_ACACIA_WOOD.clone())) };
}
lazy_static! {
    pub static ref STRIPPED_DARK_OAK_WOOD: Item =
        { Arc::new(ItemT::from(blocks::STRIPPED_DARK_OAK_WOOD.clone())) };
}
lazy_static! {
    pub static ref OAK_WOOD: Item = { Arc::new(ItemT::from(blocks::OAK_WOOD.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_WOOD: Item = { Arc::new(ItemT::from(blocks::SPRUCE_WOOD.clone())) };
}
lazy_static! {
    pub static ref BIRCH_WOOD: Item = { Arc::new(ItemT::from(blocks::BIRCH_WOOD.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_WOOD: Item = { Arc::new(ItemT::from(blocks::JUNGLE_WOOD.clone())) };
}
lazy_static! {
    pub static ref ACACIA_WOOD: Item = { Arc::new(ItemT::from(blocks::ACACIA_WOOD.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_WOOD: Item = { Arc::new(ItemT::from(blocks::DARK_OAK_WOOD.clone())) };
}
lazy_static! {
    pub static ref OAK_LEAVES: Item = { Arc::new(ItemT::from(blocks::OAK_LEAVES.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_LEAVES: Item = { Arc::new(ItemT::from(blocks::SPRUCE_LEAVES.clone())) };
}
lazy_static! {
    pub static ref BIRCH_LEAVES: Item = { Arc::new(ItemT::from(blocks::BIRCH_LEAVES.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_LEAVES: Item = { Arc::new(ItemT::from(blocks::JUNGLE_LEAVES.clone())) };
}
lazy_static! {
    pub static ref ACACIA_LEAVES: Item = { Arc::new(ItemT::from(blocks::ACACIA_LEAVES.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_LEAVES: Item =
        { Arc::new(ItemT::from(blocks::DARK_OAK_LEAVES.clone())) };
}
lazy_static! {
    pub static ref SPONGE: Item = { Arc::new(ItemT::from(blocks::SPONGE.clone())) };
}
lazy_static! {
    pub static ref WET_SPONGE: Item = { Arc::new(ItemT::from(blocks::WET_SPONGE.clone())) };
}
lazy_static! {
    pub static ref GLASS: Item = { Arc::new(ItemT::from(blocks::GLASS.clone())) };
}
lazy_static! {
    pub static ref LAPIS_ORE: Item = { Arc::new(ItemT::from(blocks::LAPIS_ORE.clone())) };
}
lazy_static! {
    pub static ref LAPIS_BLOCK: Item = { Arc::new(ItemT::from(blocks::LAPIS_BLOCK.clone())) };
}
lazy_static! {
    pub static ref DISPENSER: Item = { Arc::new(ItemT::from(blocks::DISPENSER.clone())) };
}
lazy_static! {
    pub static ref SANDSTONE: Item = { Arc::new(ItemT::from(blocks::SANDSTONE.clone())) };
}
lazy_static! {
    pub static ref CHISELED_SANDSTONE: Item =
        { Arc::new(ItemT::from(blocks::CHISELED_SANDSTONE.clone())) };
}
lazy_static! {
    pub static ref CUT_SANDSTONE: Item = { Arc::new(ItemT::from(blocks::CUT_SANDSTONE.clone())) };
}
lazy_static! {
    pub static ref NOTE_BLOCK: Item = { Arc::new(ItemT::from(blocks::NOTE_BLOCK.clone())) };
}
lazy_static! {
    pub static ref POWERED_RAIL: Item = { Arc::new(ItemT::from(blocks::POWERED_RAIL.clone())) };
}
lazy_static! {
    pub static ref DETECTOR_RAIL: Item = { Arc::new(ItemT::from(blocks::DETECTOR_RAIL.clone())) };
}
lazy_static! {
    pub static ref STICKY_PISTON: Item = { Arc::new(ItemT::from(blocks::STICKY_PISTON.clone())) };
}
lazy_static! {
    pub static ref COBWEB: Item = { Arc::new(ItemT::from(blocks::COBWEB.clone())) };
}
lazy_static! {
    pub static ref GRASS: Item = { Arc::new(ItemT::from(blocks::GRASS.clone())) };
}
lazy_static! {
    pub static ref FERN: Item = { Arc::new(ItemT::from(blocks::FERN.clone())) };
}
lazy_static! {
    pub static ref DEAD_BUSH: Item = { Arc::new(ItemT::from(blocks::DEAD_BUSH.clone())) };
}
lazy_static! {
    pub static ref SEAGRASS: Item = { Arc::new(ItemT::from(blocks::SEAGRASS.clone())) };
}
lazy_static! {
    pub static ref SEA_PICKLE: Item = { Arc::new(ItemT::from(blocks::SEA_PICKLE.clone())) };
}
lazy_static! {
    pub static ref PISTON: Item = { Arc::new(ItemT::from(blocks::PISTON.clone())) };
}
lazy_static! {
    pub static ref WHITE_WOOL: Item = { Arc::new(ItemT::from(blocks::WHITE_WOOL.clone())) };
}
lazy_static! {
    pub static ref ORANGE_WOOL: Item = { Arc::new(ItemT::from(blocks::ORANGE_WOOL.clone())) };
}
lazy_static! {
    pub static ref MAGENTA_WOOL: Item = { Arc::new(ItemT::from(blocks::MAGENTA_WOOL.clone())) };
}
lazy_static! {
    pub static ref LIGHT_BLUE_WOOL: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_BLUE_WOOL.clone())) };
}
lazy_static! {
    pub static ref YELLOW_WOOL: Item = { Arc::new(ItemT::from(blocks::YELLOW_WOOL.clone())) };
}
lazy_static! {
    pub static ref LIME_WOOL: Item = { Arc::new(ItemT::from(blocks::LIME_WOOL.clone())) };
}
lazy_static! {
    pub static ref PINK_WOOL: Item = { Arc::new(ItemT::from(blocks::PINK_WOOL.clone())) };
}
lazy_static! {
    pub static ref GRAY_WOOL: Item = { Arc::new(ItemT::from(blocks::GRAY_WOOL.clone())) };
}
lazy_static! {
    pub static ref LIGHT_GRAY_WOOL: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_GRAY_WOOL.clone())) };
}
lazy_static! {
    pub static ref CYAN_WOOL: Item = { Arc::new(ItemT::from(blocks::CYAN_WOOL.clone())) };
}
lazy_static! {
    pub static ref PURPLE_WOOL: Item = { Arc::new(ItemT::from(blocks::PURPLE_WOOL.clone())) };
}
lazy_static! {
    pub static ref BLUE_WOOL: Item = { Arc::new(ItemT::from(blocks::BLUE_WOOL.clone())) };
}
lazy_static! {
    pub static ref BROWN_WOOL: Item = { Arc::new(ItemT::from(blocks::BROWN_WOOL.clone())) };
}
lazy_static! {
    pub static ref GREEN_WOOL: Item = { Arc::new(ItemT::from(blocks::GREEN_WOOL.clone())) };
}
lazy_static! {
    pub static ref RED_WOOL: Item = { Arc::new(ItemT::from(blocks::RED_WOOL.clone())) };
}
lazy_static! {
    pub static ref BLACK_WOOL: Item = { Arc::new(ItemT::from(blocks::BLACK_WOOL.clone())) };
}
lazy_static! {
    pub static ref DANDELION: Item = { Arc::new(ItemT::from(blocks::DANDELION.clone())) };
}
lazy_static! {
    pub static ref POPPY: Item = { Arc::new(ItemT::from(blocks::POPPY.clone())) };
}
lazy_static! {
    pub static ref BLUE_ORCHID: Item = { Arc::new(ItemT::from(blocks::BLUE_ORCHID.clone())) };
}
lazy_static! {
    pub static ref ALLIUM: Item = { Arc::new(ItemT::from(blocks::ALLIUM.clone())) };
}
lazy_static! {
    pub static ref AZURE_BLUET: Item = { Arc::new(ItemT::from(blocks::AZURE_BLUET.clone())) };
}
lazy_static! {
    pub static ref RED_TULIP: Item = { Arc::new(ItemT::from(blocks::RED_TULIP.clone())) };
}
lazy_static! {
    pub static ref ORANGE_TULIP: Item = { Arc::new(ItemT::from(blocks::ORANGE_TULIP.clone())) };
}
lazy_static! {
    pub static ref WHITE_TULIP: Item = { Arc::new(ItemT::from(blocks::WHITE_TULIP.clone())) };
}
lazy_static! {
    pub static ref PINK_TULIP: Item = { Arc::new(ItemT::from(blocks::PINK_TULIP.clone())) };
}
lazy_static! {
    pub static ref OXEYE_DAISY: Item = { Arc::new(ItemT::from(blocks::OXEYE_DAISY.clone())) };
}
lazy_static! {
    pub static ref CORNFLOWER: Item = { Arc::new(ItemT::from(blocks::CORNFLOWER.clone())) };
}
lazy_static! {
    pub static ref LILY_OF_THE_VALLEY: Item =
        { Arc::new(ItemT::from(blocks::LILY_OF_THE_VALLEY.clone())) };
}
lazy_static! {
    pub static ref WITHER_ROSE: Item = { Arc::new(ItemT::from(blocks::WITHER_ROSE.clone())) };
}
lazy_static! {
    pub static ref BROWN_MUSHROOM: Item = { Arc::new(ItemT::from(blocks::BROWN_MUSHROOM.clone())) };
}
lazy_static! {
    pub static ref RED_MUSHROOM: Item = { Arc::new(ItemT::from(blocks::RED_MUSHROOM.clone())) };
}
lazy_static! {
    pub static ref GOLD_BLOCK: Item = { Arc::new(ItemT::from(blocks::GOLD_BLOCK.clone())) };
}
lazy_static! {
    pub static ref IRON_BLOCK: Item = { Arc::new(ItemT::from(blocks::IRON_BLOCK.clone())) };
}
lazy_static! {
    pub static ref OAK_SLAB: Item = { Arc::new(ItemT::from(blocks::OAK_SLAB.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_SLAB: Item = { Arc::new(ItemT::from(blocks::SPRUCE_SLAB.clone())) };
}
lazy_static! {
    pub static ref BIRCH_SLAB: Item = { Arc::new(ItemT::from(blocks::BIRCH_SLAB.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_SLAB: Item = { Arc::new(ItemT::from(blocks::JUNGLE_SLAB.clone())) };
}
lazy_static! {
    pub static ref ACACIA_SLAB: Item = { Arc::new(ItemT::from(blocks::ACACIA_SLAB.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_SLAB: Item = { Arc::new(ItemT::from(blocks::DARK_OAK_SLAB.clone())) };
}
lazy_static! {
    pub static ref STONE_SLAB: Item = { Arc::new(ItemT::from(blocks::STONE_SLAB.clone())) };
}
lazy_static! {
    pub static ref SMOOTH_STONE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::SMOOTH_STONE_SLAB.clone())) };
}
lazy_static! {
    pub static ref SANDSTONE_SLAB: Item = { Arc::new(ItemT::from(blocks::SANDSTONE_SLAB.clone())) };
}
lazy_static! {
    pub static ref CUT_SANDSTONE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::CUT_SANDSTONE_SLAB.clone())) };
}
lazy_static! {
    pub static ref PETRIFIED_OAK_SLAB: Item =
        { Arc::new(ItemT::from(blocks::PETRIFIED_OAK_SLAB.clone())) };
}
lazy_static! {
    pub static ref COBBLESTONE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::COBBLESTONE_SLAB.clone())) };
}
lazy_static! {
    pub static ref BRICK_SLAB: Item = { Arc::new(ItemT::from(blocks::BRICK_SLAB.clone())) };
}
lazy_static! {
    pub static ref STONE_BRICK_SLAB: Item =
        { Arc::new(ItemT::from(blocks::STONE_BRICK_SLAB.clone())) };
}
lazy_static! {
    pub static ref NETHER_BRICK_SLAB: Item =
        { Arc::new(ItemT::from(blocks::NETHER_BRICK_SLAB.clone())) };
}
lazy_static! {
    pub static ref QUARTZ_SLAB: Item = { Arc::new(ItemT::from(blocks::QUARTZ_SLAB.clone())) };
}
lazy_static! {
    pub static ref RED_SANDSTONE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::RED_SANDSTONE_SLAB.clone())) };
}
lazy_static! {
    pub static ref CUT_RED_SANDSTONE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::CUT_RED_SANDSTONE_SLAB.clone())) };
}
lazy_static! {
    pub static ref PURPUR_SLAB: Item = { Arc::new(ItemT::from(blocks::PURPUR_SLAB.clone())) };
}
lazy_static! {
    pub static ref PRISMARINE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::PRISMARINE_SLAB.clone())) };
}
lazy_static! {
    pub static ref PRISMARINE_BRICK_SLAB: Item =
        { Arc::new(ItemT::from(blocks::PRISMARINE_BRICK_SLAB.clone())) };
}
lazy_static! {
    pub static ref DARK_PRISMARINE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::DARK_PRISMARINE_SLAB.clone())) };
}
lazy_static! {
    pub static ref SMOOTH_QUARTZ: Item = { Arc::new(ItemT::from(blocks::SMOOTH_QUARTZ.clone())) };
}
lazy_static! {
    pub static ref SMOOTH_RED_SANDSTONE: Item =
        { Arc::new(ItemT::from(blocks::SMOOTH_RED_SANDSTONE.clone())) };
}
lazy_static! {
    pub static ref SMOOTH_SANDSTONE: Item =
        { Arc::new(ItemT::from(blocks::SMOOTH_SANDSTONE.clone())) };
}
lazy_static! {
    pub static ref SMOOTH_STONE: Item = { Arc::new(ItemT::from(blocks::SMOOTH_STONE.clone())) };
}
lazy_static! {
    pub static ref BRICKS: Item = { Arc::new(ItemT::from(blocks::BRICKS.clone())) };
}
lazy_static! {
    pub static ref TNT: Item = { Arc::new(ItemT::from(blocks::TNT.clone())) };
}
lazy_static! {
    pub static ref BOOKSHELF: Item = { Arc::new(ItemT::from(blocks::BOOKSHELF.clone())) };
}
lazy_static! {
    pub static ref MOSSY_COBBLESTONE: Item =
        { Arc::new(ItemT::from(blocks::MOSSY_COBBLESTONE.clone())) };
}
lazy_static! {
    pub static ref OBSIDIAN: Item = { Arc::new(ItemT::from(blocks::OBSIDIAN.clone())) };
}
lazy_static! {
    pub static ref TORCH: Item = { ItemT::from_block_stack_size(blocks::TORCH.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.WALL_TORCH>
lazy_static! {
    pub static ref END_ROD: Item = { Arc::new(ItemT::from(blocks::END_ROD.clone())) };
}
lazy_static! {
    pub static ref CHORUS_PLANT: Item = { Arc::new(ItemT::from(blocks::CHORUS_PLANT.clone())) };
}
lazy_static! {
    pub static ref CHORUS_FLOWER: Item = { Arc::new(ItemT::from(blocks::CHORUS_FLOWER.clone())) };
}
lazy_static! {
    pub static ref PURPUR_BLOCK: Item = { Arc::new(ItemT::from(blocks::PURPUR_BLOCK.clone())) };
}
lazy_static! {
    pub static ref PURPUR_PILLAR: Item = { Arc::new(ItemT::from(blocks::PURPUR_PILLAR.clone())) };
}
lazy_static! {
    pub static ref PURPUR_STAIRS: Item = { Arc::new(ItemT::from(blocks::PURPUR_STAIRS.clone())) };
}
lazy_static! {
    pub static ref SPAWNER: Item = { Arc::new(ItemT::from(blocks::SPAWNER.clone())) };
}
lazy_static! {
    pub static ref OAK_STAIRS: Item = { Arc::new(ItemT::from(blocks::OAK_STAIRS.clone())) };
}
lazy_static! {
    pub static ref CHEST: Item = { Arc::new(ItemT::from(blocks::CHEST.clone())) };
}
lazy_static! {
    pub static ref DIAMOND_ORE: Item = { Arc::new(ItemT::from(blocks::DIAMOND_ORE.clone())) };
}
lazy_static! {
    pub static ref DIAMOND_BLOCK: Item = { Arc::new(ItemT::from(blocks::DIAMOND_BLOCK.clone())) };
}
lazy_static! {
    pub static ref CRAFTING_TABLE: Item = { Arc::new(ItemT::from(blocks::CRAFTING_TABLE.clone())) };
}
lazy_static! {
    pub static ref FARMLAND: Item = { Arc::new(ItemT::from(blocks::FARMLAND.clone())) };
}
lazy_static! {
    pub static ref FURNACE: Item = { Arc::new(ItemT::from(blocks::FURNACE.clone())) };
}
lazy_static! {
    pub static ref LADDER: Item = { Arc::new(ItemT::from(blocks::LADDER.clone())) };
}
lazy_static! {
    pub static ref RAIL: Item = { Arc::new(ItemT::from(blocks::RAIL.clone())) };
}
lazy_static! {
    pub static ref COBBLESTONE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::COBBLESTONE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref LEVER: Item = { Arc::new(ItemT::from(blocks::LEVER.clone())) };
}
lazy_static! {
    pub static ref STONE_PRESSURE_PLATE: Item =
        { Arc::new(ItemT::from(blocks::STONE_PRESSURE_PLATE.clone())) };
}
lazy_static! {
    pub static ref OAK_PRESSURE_PLATE: Item =
        { Arc::new(ItemT::from(blocks::OAK_PRESSURE_PLATE.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_PRESSURE_PLATE: Item =
        { Arc::new(ItemT::from(blocks::SPRUCE_PRESSURE_PLATE.clone())) };
}
lazy_static! {
    pub static ref BIRCH_PRESSURE_PLATE: Item =
        { Arc::new(ItemT::from(blocks::BIRCH_PRESSURE_PLATE.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_PRESSURE_PLATE: Item =
        { Arc::new(ItemT::from(blocks::JUNGLE_PRESSURE_PLATE.clone())) };
}
lazy_static! {
    pub static ref ACACIA_PRESSURE_PLATE: Item =
        { Arc::new(ItemT::from(blocks::ACACIA_PRESSURE_PLATE.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_PRESSURE_PLATE: Item =
        { Arc::new(ItemT::from(blocks::DARK_OAK_PRESSURE_PLATE.clone())) };
}
lazy_static! {
    pub static ref REDSTONE_ORE: Item = { Arc::new(ItemT::from(blocks::REDSTONE_ORE.clone())) };
}
lazy_static! {
    pub static ref REDSTONE_TORCH: Item =
        { ItemT::from_block_stack_size(blocks::REDSTONE_TORCH.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.REDSTONE_WALL_TORCH>
lazy_static! {
    pub static ref STONE_BUTTON: Item = { Arc::new(ItemT::from(blocks::STONE_BUTTON.clone())) };
}
lazy_static! {
    pub static ref SNOW: Item = { Arc::new(ItemT::from(blocks::SNOW.clone())) };
}
lazy_static! {
    pub static ref ICE: Item = { Arc::new(ItemT::from(blocks::ICE.clone())) };
}
lazy_static! {
    pub static ref SNOW_BLOCK: Item = { Arc::new(ItemT::from(blocks::SNOW_BLOCK.clone())) };
}
lazy_static! {
    pub static ref CACTUS: Item = { Arc::new(ItemT::from(blocks::CACTUS.clone())) };
}
lazy_static! {
    pub static ref CLAY: Item = { Arc::new(ItemT::from(blocks::CLAY.clone())) };
}
lazy_static! {
    pub static ref JUKEBOX: Item = { Arc::new(ItemT::from(blocks::JUKEBOX.clone())) };
}
lazy_static! {
    pub static ref OAK_FENCE: Item = { Arc::new(ItemT::from(blocks::OAK_FENCE.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_FENCE: Item = { Arc::new(ItemT::from(blocks::SPRUCE_FENCE.clone())) };
}
lazy_static! {
    pub static ref BIRCH_FENCE: Item = { Arc::new(ItemT::from(blocks::BIRCH_FENCE.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_FENCE: Item = { Arc::new(ItemT::from(blocks::JUNGLE_FENCE.clone())) };
}
lazy_static! {
    pub static ref ACACIA_FENCE: Item = { Arc::new(ItemT::from(blocks::ACACIA_FENCE.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_FENCE: Item = { Arc::new(ItemT::from(blocks::DARK_OAK_FENCE.clone())) };
}
lazy_static! {
    pub static ref PUMPKIN: Item = { Arc::new(ItemT::from(blocks::PUMPKIN.clone())) };
}
lazy_static! {
    pub static ref CARVED_PUMPKIN: Item = { Arc::new(ItemT::from(blocks::CARVED_PUMPKIN.clone())) };
}
lazy_static! {
    pub static ref NETHERRACK: Item = { Arc::new(ItemT::from(blocks::NETHERRACK.clone())) };
}
lazy_static! {
    pub static ref SOUL_SAND: Item = { Arc::new(ItemT::from(blocks::SOUL_SAND.clone())) };
}
lazy_static! {
    pub static ref GLOWSTONE: Item = { Arc::new(ItemT::from(blocks::GLOWSTONE.clone())) };
}
lazy_static! {
    pub static ref JACK_O_LANTERN: Item = { Arc::new(ItemT::from(blocks::JACK_O_LANTERN.clone())) };
}
lazy_static! {
    pub static ref OAK_TRAPDOOR: Item = { Arc::new(ItemT::from(blocks::OAK_TRAPDOOR.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_TRAPDOOR: Item =
        { Arc::new(ItemT::from(blocks::SPRUCE_TRAPDOOR.clone())) };
}
lazy_static! {
    pub static ref BIRCH_TRAPDOOR: Item = { Arc::new(ItemT::from(blocks::BIRCH_TRAPDOOR.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_TRAPDOOR: Item =
        { Arc::new(ItemT::from(blocks::JUNGLE_TRAPDOOR.clone())) };
}
lazy_static! {
    pub static ref ACACIA_TRAPDOOR: Item =
        { Arc::new(ItemT::from(blocks::ACACIA_TRAPDOOR.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_TRAPDOOR: Item =
        { Arc::new(ItemT::from(blocks::DARK_OAK_TRAPDOOR.clone())) };
}
lazy_static! {
    pub static ref INFESTED_STONE: Item = { Arc::new(ItemT::from(blocks::INFESTED_STONE.clone())) };
}
lazy_static! {
    pub static ref INFESTED_COBBLESTONE: Item =
        { Arc::new(ItemT::from(blocks::INFESTED_COBBLESTONE.clone())) };
}
lazy_static! {
    pub static ref INFESTED_STONE_BRICKS: Item =
        { Arc::new(ItemT::from(blocks::INFESTED_STONE_BRICKS.clone())) };
}
lazy_static! {
    pub static ref INFESTED_MOSSY_STONE_BRICKS: Item =
        { Arc::new(ItemT::from(blocks::INFESTED_MOSSY_STONE_BRICKS.clone())) };
}
lazy_static! {
    pub static ref INFESTED_CRACKED_STONE_BRICKS: Item =
        { Arc::new(ItemT::from(blocks::INFESTED_CRACKED_STONE_BRICKS.clone())) };
}
lazy_static! {
    pub static ref INFESTED_CHISELED_STONE_BRICKS: Item =
        { Arc::new(ItemT::from(blocks::INFESTED_CHISELED_STONE_BRICKS.clone())) };
}
lazy_static! {
    pub static ref STONE_BRICKS: Item = { Arc::new(ItemT::from(blocks::STONE_BRICKS.clone())) };
}
lazy_static! {
    pub static ref MOSSY_STONE_BRICKS: Item =
        { Arc::new(ItemT::from(blocks::MOSSY_STONE_BRICKS.clone())) };
}
lazy_static! {
    pub static ref CRACKED_STONE_BRICKS: Item =
        { Arc::new(ItemT::from(blocks::CRACKED_STONE_BRICKS.clone())) };
}
lazy_static! {
    pub static ref CHISELED_STONE_BRICKS: Item =
        { Arc::new(ItemT::from(blocks::CHISELED_STONE_BRICKS.clone())) };
}
lazy_static! {
    pub static ref BROWN_MUSHROOM_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::BROWN_MUSHROOM_BLOCK.clone())) };
}
lazy_static! {
    pub static ref RED_MUSHROOM_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::RED_MUSHROOM_BLOCK.clone())) };
}
lazy_static! {
    pub static ref MUSHROOM_STEM: Item = { Arc::new(ItemT::from(blocks::MUSHROOM_STEM.clone())) };
}
lazy_static! {
    pub static ref IRON_BARS: Item = { Arc::new(ItemT::from(blocks::IRON_BARS.clone())) };
}
lazy_static! {
    pub static ref GLASS_PANE: Item = { Arc::new(ItemT::from(blocks::GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref MELON: Item = { Arc::new(ItemT::from(blocks::MELON.clone())) };
}
lazy_static! {
    pub static ref VINE: Item = { Arc::new(ItemT::from(blocks::VINE.clone())) };
}
lazy_static! {
    pub static ref OAK_FENCE_GATE: Item = { Arc::new(ItemT::from(blocks::OAK_FENCE_GATE.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_FENCE_GATE: Item =
        { Arc::new(ItemT::from(blocks::SPRUCE_FENCE_GATE.clone())) };
}
lazy_static! {
    pub static ref BIRCH_FENCE_GATE: Item =
        { Arc::new(ItemT::from(blocks::BIRCH_FENCE_GATE.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_FENCE_GATE: Item =
        { Arc::new(ItemT::from(blocks::JUNGLE_FENCE_GATE.clone())) };
}
lazy_static! {
    pub static ref ACACIA_FENCE_GATE: Item =
        { Arc::new(ItemT::from(blocks::ACACIA_FENCE_GATE.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_FENCE_GATE: Item =
        { Arc::new(ItemT::from(blocks::DARK_OAK_FENCE_GATE.clone())) };
}
lazy_static! {
    pub static ref BRICK_STAIRS: Item = { Arc::new(ItemT::from(blocks::BRICK_STAIRS.clone())) };
}
lazy_static! {
    pub static ref STONE_BRICK_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::STONE_BRICK_STAIRS.clone())) };
}
lazy_static! {
    pub static ref MYCELIUM: Item = { Arc::new(ItemT::from(blocks::MYCELIUM.clone())) };
}
lazy_static! {
    pub static ref LILY_PAD: Item = { ItemT::from_block_stack_size(blocks::LILY_PAD.clone(), 64) };
} // WaterLilyBlockItem<>
lazy_static! {
    pub static ref NETHER_BRICKS: Item = { Arc::new(ItemT::from(blocks::NETHER_BRICKS.clone())) };
}
lazy_static! {
    pub static ref NETHER_BRICK_FENCE: Item =
        { Arc::new(ItemT::from(blocks::NETHER_BRICK_FENCE.clone())) };
}
lazy_static! {
    pub static ref NETHER_BRICK_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::NETHER_BRICK_STAIRS.clone())) };
}
lazy_static! {
    pub static ref ENCHANTING_TABLE: Item =
        { Arc::new(ItemT::from(blocks::ENCHANTING_TABLE.clone())) };
}
lazy_static! {
    pub static ref END_PORTAL_FRAME: Item =
        { Arc::new(ItemT::from(blocks::END_PORTAL_FRAME.clone())) };
}
lazy_static! {
    pub static ref END_STONE: Item = { Arc::new(ItemT::from(blocks::END_STONE.clone())) };
}
lazy_static! {
    pub static ref END_STONE_BRICKS: Item =
        { Arc::new(ItemT::from(blocks::END_STONE_BRICKS.clone())) };
}
lazy_static! {
    pub static ref DRAGON_EGG: Item =
        { ItemT::from_block_stack_size(blocks::DRAGON_EGG.clone(), 64) };
} // BlockItem<>
lazy_static! {
    pub static ref REDSTONE_LAMP: Item = { Arc::new(ItemT::from(blocks::REDSTONE_LAMP.clone())) };
}
lazy_static! {
    pub static ref SANDSTONE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::SANDSTONE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref EMERALD_ORE: Item = { Arc::new(ItemT::from(blocks::EMERALD_ORE.clone())) };
}
lazy_static! {
    pub static ref ENDER_CHEST: Item = { Arc::new(ItemT::from(blocks::ENDER_CHEST.clone())) };
}
lazy_static! {
    pub static ref TRIPWIRE_HOOK: Item = { Arc::new(ItemT::from(blocks::TRIPWIRE_HOOK.clone())) };
}
lazy_static! {
    pub static ref EMERALD_BLOCK: Item = { Arc::new(ItemT::from(blocks::EMERALD_BLOCK.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_STAIRS: Item = { Arc::new(ItemT::from(blocks::SPRUCE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref BIRCH_STAIRS: Item = { Arc::new(ItemT::from(blocks::BIRCH_STAIRS.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_STAIRS: Item = { Arc::new(ItemT::from(blocks::JUNGLE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref COMMAND_BLOCK: Item =
        { ItemT::from_block_stack_size(blocks::COMMAND_BLOCK.clone(), 64) };
} // GameMasterBlockItem<>
lazy_static! {
    pub static ref BEACON: Item = { ItemT::from_block_stack_size(blocks::BEACON.clone(), 64) };
} // BlockItem<>
lazy_static! {
    pub static ref COBBLESTONE_WALL: Item =
        { Arc::new(ItemT::from(blocks::COBBLESTONE_WALL.clone())) };
}
lazy_static! {
    pub static ref MOSSY_COBBLESTONE_WALL: Item =
        { Arc::new(ItemT::from(blocks::MOSSY_COBBLESTONE_WALL.clone())) };
}
lazy_static! {
    pub static ref BRICK_WALL: Item = { Arc::new(ItemT::from(blocks::BRICK_WALL.clone())) };
}
lazy_static! {
    pub static ref PRISMARINE_WALL: Item =
        { Arc::new(ItemT::from(blocks::PRISMARINE_WALL.clone())) };
}
lazy_static! {
    pub static ref RED_SANDSTONE_WALL: Item =
        { Arc::new(ItemT::from(blocks::RED_SANDSTONE_WALL.clone())) };
}
lazy_static! {
    pub static ref MOSSY_STONE_BRICK_WALL: Item =
        { Arc::new(ItemT::from(blocks::MOSSY_STONE_BRICK_WALL.clone())) };
}
lazy_static! {
    pub static ref GRANITE_WALL: Item = { Arc::new(ItemT::from(blocks::GRANITE_WALL.clone())) };
}
lazy_static! {
    pub static ref STONE_BRICK_WALL: Item =
        { Arc::new(ItemT::from(blocks::STONE_BRICK_WALL.clone())) };
}
lazy_static! {
    pub static ref NETHER_BRICK_WALL: Item =
        { Arc::new(ItemT::from(blocks::NETHER_BRICK_WALL.clone())) };
}
lazy_static! {
    pub static ref ANDESITE_WALL: Item = { Arc::new(ItemT::from(blocks::ANDESITE_WALL.clone())) };
}
lazy_static! {
    pub static ref RED_NETHER_BRICK_WALL: Item =
        { Arc::new(ItemT::from(blocks::RED_NETHER_BRICK_WALL.clone())) };
}
lazy_static! {
    pub static ref SANDSTONE_WALL: Item = { Arc::new(ItemT::from(blocks::SANDSTONE_WALL.clone())) };
}
lazy_static! {
    pub static ref END_STONE_BRICK_WALL: Item =
        { Arc::new(ItemT::from(blocks::END_STONE_BRICK_WALL.clone())) };
}
lazy_static! {
    pub static ref DIORITE_WALL: Item = { Arc::new(ItemT::from(blocks::DIORITE_WALL.clone())) };
}
lazy_static! {
    pub static ref OAK_BUTTON: Item = { Arc::new(ItemT::from(blocks::OAK_BUTTON.clone())) };
}
lazy_static! {
    pub static ref SPRUCE_BUTTON: Item = { Arc::new(ItemT::from(blocks::SPRUCE_BUTTON.clone())) };
}
lazy_static! {
    pub static ref BIRCH_BUTTON: Item = { Arc::new(ItemT::from(blocks::BIRCH_BUTTON.clone())) };
}
lazy_static! {
    pub static ref JUNGLE_BUTTON: Item = { Arc::new(ItemT::from(blocks::JUNGLE_BUTTON.clone())) };
}
lazy_static! {
    pub static ref ACACIA_BUTTON: Item = { Arc::new(ItemT::from(blocks::ACACIA_BUTTON.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_BUTTON: Item =
        { Arc::new(ItemT::from(blocks::DARK_OAK_BUTTON.clone())) };
}
lazy_static! {
    pub static ref ANVIL: Item = { Arc::new(ItemT::from(blocks::ANVIL.clone())) };
}
lazy_static! {
    pub static ref CHIPPED_ANVIL: Item = { Arc::new(ItemT::from(blocks::CHIPPED_ANVIL.clone())) };
}
lazy_static! {
    pub static ref DAMAGED_ANVIL: Item = { Arc::new(ItemT::from(blocks::DAMAGED_ANVIL.clone())) };
}
lazy_static! {
    pub static ref TRAPPED_CHEST: Item = { Arc::new(ItemT::from(blocks::TRAPPED_CHEST.clone())) };
}
lazy_static! {
    pub static ref LIGHT_WEIGHTED_PRESSURE_PLATE: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_WEIGHTED_PRESSURE_PLATE.clone())) };
}
lazy_static! {
    pub static ref HEAVY_WEIGHTED_PRESSURE_PLATE: Item =
        { Arc::new(ItemT::from(blocks::HEAVY_WEIGHTED_PRESSURE_PLATE.clone())) };
}
lazy_static! {
    pub static ref DAYLIGHT_DETECTOR: Item =
        { Arc::new(ItemT::from(blocks::DAYLIGHT_DETECTOR.clone())) };
}
lazy_static! {
    pub static ref REDSTONE_BLOCK: Item = { Arc::new(ItemT::from(blocks::REDSTONE_BLOCK.clone())) };
}
lazy_static! {
    pub static ref NETHER_QUARTZ_ORE: Item =
        { Arc::new(ItemT::from(blocks::NETHER_QUARTZ_ORE.clone())) };
}
lazy_static! {
    pub static ref HOPPER: Item = { Arc::new(ItemT::from(blocks::HOPPER.clone())) };
}
lazy_static! {
    pub static ref CHISELED_QUARTZ_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::CHISELED_QUARTZ_BLOCK.clone())) };
}
lazy_static! {
    pub static ref QUARTZ_BLOCK: Item = { Arc::new(ItemT::from(blocks::QUARTZ_BLOCK.clone())) };
}
lazy_static! {
    pub static ref QUARTZ_PILLAR: Item = { Arc::new(ItemT::from(blocks::QUARTZ_PILLAR.clone())) };
}
lazy_static! {
    pub static ref QUARTZ_STAIRS: Item = { Arc::new(ItemT::from(blocks::QUARTZ_STAIRS.clone())) };
}
lazy_static! {
    pub static ref ACTIVATOR_RAIL: Item = { Arc::new(ItemT::from(blocks::ACTIVATOR_RAIL.clone())) };
}
lazy_static! {
    pub static ref DROPPER: Item = { Arc::new(ItemT::from(blocks::DROPPER.clone())) };
}
lazy_static! {
    pub static ref WHITE_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::WHITE_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref ORANGE_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::ORANGE_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref MAGENTA_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::MAGENTA_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref LIGHT_BLUE_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_BLUE_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref YELLOW_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::YELLOW_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref LIME_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::LIME_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref PINK_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::PINK_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref GRAY_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::GRAY_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref LIGHT_GRAY_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_GRAY_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref CYAN_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::CYAN_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref PURPLE_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::PURPLE_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref BLUE_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::BLUE_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref BROWN_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::BROWN_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref GREEN_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::GREEN_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref RED_TERRACOTTA: Item = { Arc::new(ItemT::from(blocks::RED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref BLACK_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::BLACK_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref BARRIER: Item = { Arc::new(ItemT::from(blocks::BARRIER.clone())) };
}
lazy_static! {
    pub static ref IRON_TRAPDOOR: Item = { Arc::new(ItemT::from(blocks::IRON_TRAPDOOR.clone())) };
}
lazy_static! {
    pub static ref HAY_BLOCK: Item = { Arc::new(ItemT::from(blocks::HAY_BLOCK.clone())) };
}
lazy_static! {
    pub static ref WHITE_CARPET: Item = { Arc::new(ItemT::from(blocks::WHITE_CARPET.clone())) };
}
lazy_static! {
    pub static ref ORANGE_CARPET: Item = { Arc::new(ItemT::from(blocks::ORANGE_CARPET.clone())) };
}
lazy_static! {
    pub static ref MAGENTA_CARPET: Item = { Arc::new(ItemT::from(blocks::MAGENTA_CARPET.clone())) };
}
lazy_static! {
    pub static ref LIGHT_BLUE_CARPET: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_BLUE_CARPET.clone())) };
}
lazy_static! {
    pub static ref YELLOW_CARPET: Item = { Arc::new(ItemT::from(blocks::YELLOW_CARPET.clone())) };
}
lazy_static! {
    pub static ref LIME_CARPET: Item = { Arc::new(ItemT::from(blocks::LIME_CARPET.clone())) };
}
lazy_static! {
    pub static ref PINK_CARPET: Item = { Arc::new(ItemT::from(blocks::PINK_CARPET.clone())) };
}
lazy_static! {
    pub static ref GRAY_CARPET: Item = { Arc::new(ItemT::from(blocks::GRAY_CARPET.clone())) };
}
lazy_static! {
    pub static ref LIGHT_GRAY_CARPET: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_GRAY_CARPET.clone())) };
}
lazy_static! {
    pub static ref CYAN_CARPET: Item = { Arc::new(ItemT::from(blocks::CYAN_CARPET.clone())) };
}
lazy_static! {
    pub static ref PURPLE_CARPET: Item = { Arc::new(ItemT::from(blocks::PURPLE_CARPET.clone())) };
}
lazy_static! {
    pub static ref BLUE_CARPET: Item = { Arc::new(ItemT::from(blocks::BLUE_CARPET.clone())) };
}
lazy_static! {
    pub static ref BROWN_CARPET: Item = { Arc::new(ItemT::from(blocks::BROWN_CARPET.clone())) };
}
lazy_static! {
    pub static ref GREEN_CARPET: Item = { Arc::new(ItemT::from(blocks::GREEN_CARPET.clone())) };
}
lazy_static! {
    pub static ref RED_CARPET: Item = { Arc::new(ItemT::from(blocks::RED_CARPET.clone())) };
}
lazy_static! {
    pub static ref BLACK_CARPET: Item = { Arc::new(ItemT::from(blocks::BLACK_CARPET.clone())) };
}
lazy_static! {
    pub static ref TERRACOTTA: Item = { Arc::new(ItemT::from(blocks::TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref COAL_BLOCK: Item = { Arc::new(ItemT::from(blocks::COAL_BLOCK.clone())) };
}
lazy_static! {
    pub static ref PACKED_ICE: Item = { Arc::new(ItemT::from(blocks::PACKED_ICE.clone())) };
}
lazy_static! {
    pub static ref ACACIA_STAIRS: Item = { Arc::new(ItemT::from(blocks::ACACIA_STAIRS.clone())) };
}
lazy_static! {
    pub static ref DARK_OAK_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::DARK_OAK_STAIRS.clone())) };
}
lazy_static! {
    pub static ref SLIME_BLOCK: Item = { Arc::new(ItemT::from(blocks::SLIME_BLOCK.clone())) };
}
lazy_static! {
    pub static ref GRASS_PATH: Item = { Arc::new(ItemT::from(blocks::GRASS_PATH.clone())) };
}
lazy_static! {
    pub static ref SUNFLOWER: Item =
        { ItemT::from_block_stack_size(blocks::SUNFLOWER.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref LILAC: Item = { ItemT::from_block_stack_size(blocks::LILAC.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref ROSE_BUSH: Item =
        { ItemT::from_block_stack_size(blocks::ROSE_BUSH.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref PEONY: Item = { ItemT::from_block_stack_size(blocks::PEONY.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref TALL_GRASS: Item =
        { ItemT::from_block_stack_size(blocks::TALL_GRASS.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref LARGE_FERN: Item =
        { ItemT::from_block_stack_size(blocks::LARGE_FERN.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref WHITE_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::WHITE_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref ORANGE_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::ORANGE_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref MAGENTA_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::MAGENTA_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref LIGHT_BLUE_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_BLUE_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref YELLOW_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::YELLOW_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref LIME_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::LIME_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref PINK_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::PINK_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref GRAY_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::GRAY_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref LIGHT_GRAY_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_GRAY_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref CYAN_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::CYAN_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref PURPLE_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::PURPLE_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref BLUE_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::BLUE_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref BROWN_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::BROWN_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref GREEN_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::GREEN_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref RED_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::RED_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref BLACK_STAINED_GLASS: Item =
        { Arc::new(ItemT::from(blocks::BLACK_STAINED_GLASS.clone())) };
}
lazy_static! {
    pub static ref WHITE_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::WHITE_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref ORANGE_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::ORANGE_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref MAGENTA_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::MAGENTA_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref LIGHT_BLUE_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_BLUE_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref YELLOW_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::YELLOW_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref LIME_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::LIME_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref PINK_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::PINK_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref GRAY_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::GRAY_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref LIGHT_GRAY_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_GRAY_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref CYAN_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::CYAN_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref PURPLE_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::PURPLE_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref BLUE_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::BLUE_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref BROWN_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::BROWN_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref GREEN_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::GREEN_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref RED_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::RED_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref BLACK_STAINED_GLASS_PANE: Item =
        { Arc::new(ItemT::from(blocks::BLACK_STAINED_GLASS_PANE.clone())) };
}
lazy_static! {
    pub static ref PRISMARINE: Item = { Arc::new(ItemT::from(blocks::PRISMARINE.clone())) };
}
lazy_static! {
    pub static ref PRISMARINE_BRICKS: Item =
        { Arc::new(ItemT::from(blocks::PRISMARINE_BRICKS.clone())) };
}
lazy_static! {
    pub static ref DARK_PRISMARINE: Item =
        { Arc::new(ItemT::from(blocks::DARK_PRISMARINE.clone())) };
}
lazy_static! {
    pub static ref PRISMARINE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::PRISMARINE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref PRISMARINE_BRICK_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::PRISMARINE_BRICK_STAIRS.clone())) };
}
lazy_static! {
    pub static ref DARK_PRISMARINE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::DARK_PRISMARINE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref SEA_LANTERN: Item = { Arc::new(ItemT::from(blocks::SEA_LANTERN.clone())) };
}
lazy_static! {
    pub static ref RED_SANDSTONE: Item = { Arc::new(ItemT::from(blocks::RED_SANDSTONE.clone())) };
}
lazy_static! {
    pub static ref CHISELED_RED_SANDSTONE: Item =
        { Arc::new(ItemT::from(blocks::CHISELED_RED_SANDSTONE.clone())) };
}
lazy_static! {
    pub static ref CUT_RED_SANDSTONE: Item =
        { Arc::new(ItemT::from(blocks::CUT_RED_SANDSTONE.clone())) };
}
lazy_static! {
    pub static ref RED_SANDSTONE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::RED_SANDSTONE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref REPEATING_COMMAND_BLOCK: Item =
        { ItemT::from_block_stack_size(blocks::REPEATING_COMMAND_BLOCK.clone(), 64) };
} // GameMasterBlockItem<>
lazy_static! {
    pub static ref CHAIN_COMMAND_BLOCK: Item =
        { ItemT::from_block_stack_size(blocks::CHAIN_COMMAND_BLOCK.clone(), 64) };
} // GameMasterBlockItem<>
lazy_static! {
    pub static ref MAGMA_BLOCK: Item = { Arc::new(ItemT::from(blocks::MAGMA_BLOCK.clone())) };
}
lazy_static! {
    pub static ref NETHER_WART_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::NETHER_WART_BLOCK.clone())) };
}
lazy_static! {
    pub static ref RED_NETHER_BRICKS: Item =
        { Arc::new(ItemT::from(blocks::RED_NETHER_BRICKS.clone())) };
}
lazy_static! {
    pub static ref BONE_BLOCK: Item = { Arc::new(ItemT::from(blocks::BONE_BLOCK.clone())) };
}
lazy_static! {
    pub static ref STRUCTURE_VOID: Item = { Arc::new(ItemT::from(blocks::STRUCTURE_VOID.clone())) };
}
lazy_static! {
    pub static ref OBSERVER: Item = { Arc::new(ItemT::from(blocks::OBSERVER.clone())) };
}
lazy_static! {
    pub static ref SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref WHITE_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::WHITE_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref ORANGE_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::ORANGE_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref MAGENTA_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::MAGENTA_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref LIGHT_BLUE_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::LIGHT_BLUE_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref YELLOW_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::YELLOW_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref LIME_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::LIME_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref PINK_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::PINK_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref GRAY_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::GRAY_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref LIGHT_GRAY_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::LIGHT_GRAY_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref CYAN_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::CYAN_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref PURPLE_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::PURPLE_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref BLUE_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::BLUE_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref BROWN_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::BROWN_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref GREEN_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::GREEN_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref RED_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::RED_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref BLACK_SHULKER_BOX: Item =
        { ItemT::from_block_stack_size(blocks::BLACK_SHULKER_BOX.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref WHITE_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::WHITE_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref ORANGE_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::ORANGE_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref MAGENTA_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::MAGENTA_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref LIGHT_BLUE_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_BLUE_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref YELLOW_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::YELLOW_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref LIME_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::LIME_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref PINK_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::PINK_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref GRAY_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::GRAY_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref LIGHT_GRAY_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_GRAY_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref CYAN_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::CYAN_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref PURPLE_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::PURPLE_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref BLUE_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::BLUE_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref BROWN_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::BROWN_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref GREEN_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::GREEN_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref RED_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::RED_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref BLACK_GLAZED_TERRACOTTA: Item =
        { Arc::new(ItemT::from(blocks::BLACK_GLAZED_TERRACOTTA.clone())) };
}
lazy_static! {
    pub static ref WHITE_CONCRETE: Item = { Arc::new(ItemT::from(blocks::WHITE_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref ORANGE_CONCRETE: Item =
        { Arc::new(ItemT::from(blocks::ORANGE_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref MAGENTA_CONCRETE: Item =
        { Arc::new(ItemT::from(blocks::MAGENTA_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref LIGHT_BLUE_CONCRETE: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_BLUE_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref YELLOW_CONCRETE: Item =
        { Arc::new(ItemT::from(blocks::YELLOW_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref LIME_CONCRETE: Item = { Arc::new(ItemT::from(blocks::LIME_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref PINK_CONCRETE: Item = { Arc::new(ItemT::from(blocks::PINK_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref GRAY_CONCRETE: Item = { Arc::new(ItemT::from(blocks::GRAY_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref LIGHT_GRAY_CONCRETE: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_GRAY_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref CYAN_CONCRETE: Item = { Arc::new(ItemT::from(blocks::CYAN_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref PURPLE_CONCRETE: Item =
        { Arc::new(ItemT::from(blocks::PURPLE_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref BLUE_CONCRETE: Item = { Arc::new(ItemT::from(blocks::BLUE_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref BROWN_CONCRETE: Item = { Arc::new(ItemT::from(blocks::BROWN_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref GREEN_CONCRETE: Item = { Arc::new(ItemT::from(blocks::GREEN_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref RED_CONCRETE: Item = { Arc::new(ItemT::from(blocks::RED_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref BLACK_CONCRETE: Item = { Arc::new(ItemT::from(blocks::BLACK_CONCRETE.clone())) };
}
lazy_static! {
    pub static ref WHITE_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::WHITE_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref ORANGE_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::ORANGE_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref MAGENTA_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::MAGENTA_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref LIGHT_BLUE_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_BLUE_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref YELLOW_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::YELLOW_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref LIME_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::LIME_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref PINK_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::PINK_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref GRAY_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::GRAY_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref LIGHT_GRAY_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::LIGHT_GRAY_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref CYAN_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::CYAN_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref PURPLE_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::PURPLE_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref BLUE_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::BLUE_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref BROWN_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::BROWN_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref GREEN_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::GREEN_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref RED_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::RED_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref BLACK_CONCRETE_POWDER: Item =
        { Arc::new(ItemT::from(blocks::BLACK_CONCRETE_POWDER.clone())) };
}
lazy_static! {
    pub static ref TURTLE_EGG: Item = { Arc::new(ItemT::from(blocks::TURTLE_EGG.clone())) };
}
lazy_static! {
    pub static ref DEAD_TUBE_CORAL_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::DEAD_TUBE_CORAL_BLOCK.clone())) };
}
lazy_static! {
    pub static ref DEAD_BRAIN_CORAL_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::DEAD_BRAIN_CORAL_BLOCK.clone())) };
}
lazy_static! {
    pub static ref DEAD_BUBBLE_CORAL_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::DEAD_BUBBLE_CORAL_BLOCK.clone())) };
}
lazy_static! {
    pub static ref DEAD_FIRE_CORAL_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::DEAD_FIRE_CORAL_BLOCK.clone())) };
}
lazy_static! {
    pub static ref DEAD_HORN_CORAL_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::DEAD_HORN_CORAL_BLOCK.clone())) };
}
lazy_static! {
    pub static ref TUBE_CORAL_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::TUBE_CORAL_BLOCK.clone())) };
}
lazy_static! {
    pub static ref BRAIN_CORAL_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::BRAIN_CORAL_BLOCK.clone())) };
}
lazy_static! {
    pub static ref BUBBLE_CORAL_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::BUBBLE_CORAL_BLOCK.clone())) };
}
lazy_static! {
    pub static ref FIRE_CORAL_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::FIRE_CORAL_BLOCK.clone())) };
}
lazy_static! {
    pub static ref HORN_CORAL_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::HORN_CORAL_BLOCK.clone())) };
}
lazy_static! {
    pub static ref TUBE_CORAL: Item = { Arc::new(ItemT::from(blocks::TUBE_CORAL.clone())) };
}
lazy_static! {
    pub static ref BRAIN_CORAL: Item = { Arc::new(ItemT::from(blocks::BRAIN_CORAL.clone())) };
}
lazy_static! {
    pub static ref BUBBLE_CORAL: Item = { Arc::new(ItemT::from(blocks::BUBBLE_CORAL.clone())) };
}
lazy_static! {
    pub static ref FIRE_CORAL: Item = { Arc::new(ItemT::from(blocks::FIRE_CORAL.clone())) };
}
lazy_static! {
    pub static ref HORN_CORAL: Item = { Arc::new(ItemT::from(blocks::HORN_CORAL.clone())) };
}
lazy_static! {
    pub static ref DEAD_BRAIN_CORAL: Item =
        { Arc::new(ItemT::from(blocks::DEAD_BRAIN_CORAL.clone())) };
}
lazy_static! {
    pub static ref DEAD_BUBBLE_CORAL: Item =
        { Arc::new(ItemT::from(blocks::DEAD_BUBBLE_CORAL.clone())) };
}
lazy_static! {
    pub static ref DEAD_FIRE_CORAL: Item =
        { Arc::new(ItemT::from(blocks::DEAD_FIRE_CORAL.clone())) };
}
lazy_static! {
    pub static ref DEAD_HORN_CORAL: Item =
        { Arc::new(ItemT::from(blocks::DEAD_HORN_CORAL.clone())) };
}
lazy_static! {
    pub static ref DEAD_TUBE_CORAL: Item =
        { Arc::new(ItemT::from(blocks::DEAD_TUBE_CORAL.clone())) };
}
lazy_static! {
    pub static ref TUBE_CORAL_FAN: Item =
        { ItemT::from_block_stack_size(blocks::TUBE_CORAL_FAN.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.TUBE_CORAL_WALL_FAN>
lazy_static! {
    pub static ref BRAIN_CORAL_FAN: Item =
        { ItemT::from_block_stack_size(blocks::BRAIN_CORAL_FAN.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.BRAIN_CORAL_WALL_FAN>
lazy_static! {
    pub static ref BUBBLE_CORAL_FAN: Item =
        { ItemT::from_block_stack_size(blocks::BUBBLE_CORAL_FAN.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.BUBBLE_CORAL_WALL_FAN>
lazy_static! {
    pub static ref FIRE_CORAL_FAN: Item =
        { ItemT::from_block_stack_size(blocks::FIRE_CORAL_FAN.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.FIRE_CORAL_WALL_FAN>
lazy_static! {
    pub static ref HORN_CORAL_FAN: Item =
        { ItemT::from_block_stack_size(blocks::HORN_CORAL_FAN.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.HORN_CORAL_WALL_FAN>
lazy_static! {
    pub static ref DEAD_TUBE_CORAL_FAN: Item =
        { ItemT::from_block_stack_size(blocks::DEAD_TUBE_CORAL_FAN.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.DEAD_TUBE_CORAL_WALL_FAN>
lazy_static! {
    pub static ref DEAD_BRAIN_CORAL_FAN: Item =
        { ItemT::from_block_stack_size(blocks::DEAD_BRAIN_CORAL_FAN.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.DEAD_BRAIN_CORAL_WALL_FAN>
lazy_static! {
    pub static ref DEAD_BUBBLE_CORAL_FAN: Item =
        { ItemT::from_block_stack_size(blocks::DEAD_BUBBLE_CORAL_FAN.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.DEAD_BUBBLE_CORAL_WALL_FAN>
lazy_static! {
    pub static ref DEAD_FIRE_CORAL_FAN: Item =
        { ItemT::from_block_stack_size(blocks::DEAD_FIRE_CORAL_FAN.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.DEAD_FIRE_CORAL_WALL_FAN>
lazy_static! {
    pub static ref DEAD_HORN_CORAL_FAN: Item =
        { ItemT::from_block_stack_size(blocks::DEAD_HORN_CORAL_FAN.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.DEAD_HORN_CORAL_WALL_FAN>
lazy_static! {
    pub static ref BLUE_ICE: Item = { Arc::new(ItemT::from(blocks::BLUE_ICE.clone())) };
}
lazy_static! {
    pub static ref CONDUIT: Item = { ItemT::from_block_stack_size(blocks::CONDUIT.clone(), 64) };
} // BlockItem<>
lazy_static! {
    pub static ref POLISHED_GRANITE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::POLISHED_GRANITE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref SMOOTH_RED_SANDSTONE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::SMOOTH_RED_SANDSTONE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref MOSSY_STONE_BRICK_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::MOSSY_STONE_BRICK_STAIRS.clone())) };
}
lazy_static! {
    pub static ref POLISHED_DIORITE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::POLISHED_DIORITE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref MOSSY_COBBLESTONE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::MOSSY_COBBLESTONE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref END_STONE_BRICK_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::END_STONE_BRICK_STAIRS.clone())) };
}
lazy_static! {
    pub static ref STONE_STAIRS: Item = { Arc::new(ItemT::from(blocks::STONE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref SMOOTH_SANDSTONE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::SMOOTH_SANDSTONE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref SMOOTH_QUARTZ_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::SMOOTH_QUARTZ_STAIRS.clone())) };
}
lazy_static! {
    pub static ref GRANITE_STAIRS: Item = { Arc::new(ItemT::from(blocks::GRANITE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref ANDESITE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::ANDESITE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref RED_NETHER_BRICK_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::RED_NETHER_BRICK_STAIRS.clone())) };
}
lazy_static! {
    pub static ref POLISHED_ANDESITE_STAIRS: Item =
        { Arc::new(ItemT::from(blocks::POLISHED_ANDESITE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref DIORITE_STAIRS: Item = { Arc::new(ItemT::from(blocks::DIORITE_STAIRS.clone())) };
}
lazy_static! {
    pub static ref POLISHED_GRANITE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::POLISHED_GRANITE_SLAB.clone())) };
}
lazy_static! {
    pub static ref SMOOTH_RED_SANDSTONE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::SMOOTH_RED_SANDSTONE_SLAB.clone())) };
}
lazy_static! {
    pub static ref MOSSY_STONE_BRICK_SLAB: Item =
        { Arc::new(ItemT::from(blocks::MOSSY_STONE_BRICK_SLAB.clone())) };
}
lazy_static! {
    pub static ref POLISHED_DIORITE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::POLISHED_DIORITE_SLAB.clone())) };
}
lazy_static! {
    pub static ref MOSSY_COBBLESTONE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::MOSSY_COBBLESTONE_SLAB.clone())) };
}
lazy_static! {
    pub static ref END_STONE_BRICK_SLAB: Item =
        { Arc::new(ItemT::from(blocks::END_STONE_BRICK_SLAB.clone())) };
}
lazy_static! {
    pub static ref SMOOTH_SANDSTONE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::SMOOTH_SANDSTONE_SLAB.clone())) };
}
lazy_static! {
    pub static ref SMOOTH_QUARTZ_SLAB: Item =
        { Arc::new(ItemT::from(blocks::SMOOTH_QUARTZ_SLAB.clone())) };
}
lazy_static! {
    pub static ref GRANITE_SLAB: Item = { Arc::new(ItemT::from(blocks::GRANITE_SLAB.clone())) };
}
lazy_static! {
    pub static ref ANDESITE_SLAB: Item = { Arc::new(ItemT::from(blocks::ANDESITE_SLAB.clone())) };
}
lazy_static! {
    pub static ref RED_NETHER_BRICK_SLAB: Item =
        { Arc::new(ItemT::from(blocks::RED_NETHER_BRICK_SLAB.clone())) };
}
lazy_static! {
    pub static ref POLISHED_ANDESITE_SLAB: Item =
        { Arc::new(ItemT::from(blocks::POLISHED_ANDESITE_SLAB.clone())) };
}
lazy_static! {
    pub static ref DIORITE_SLAB: Item = { Arc::new(ItemT::from(blocks::DIORITE_SLAB.clone())) };
}
lazy_static! {
    pub static ref SCAFFOLDING: Item =
        { ItemT::from_block_stack_size(blocks::SCAFFOLDING.clone(), 64) };
} // ScaffoldingBlockItem<>
lazy_static! {
    pub static ref IRON_DOOR: Item =
        { ItemT::from_block_stack_size(blocks::IRON_DOOR.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref OAK_DOOR: Item = { ItemT::from_block_stack_size(blocks::OAK_DOOR.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref SPRUCE_DOOR: Item =
        { ItemT::from_block_stack_size(blocks::SPRUCE_DOOR.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref BIRCH_DOOR: Item =
        { ItemT::from_block_stack_size(blocks::BIRCH_DOOR.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref JUNGLE_DOOR: Item =
        { ItemT::from_block_stack_size(blocks::JUNGLE_DOOR.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref ACACIA_DOOR: Item =
        { ItemT::from_block_stack_size(blocks::ACACIA_DOOR.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref DARK_OAK_DOOR: Item =
        { ItemT::from_block_stack_size(blocks::DARK_OAK_DOOR.clone(), 64) };
} // DoubleHighBlockItem<>
lazy_static! {
    pub static ref REPEATER: Item = { Arc::new(ItemT::from(blocks::REPEATER.clone())) };
}
lazy_static! {
    pub static ref COMPARATOR: Item = { Arc::new(ItemT::from(blocks::COMPARATOR.clone())) };
}
lazy_static! {
    pub static ref STRUCTURE_BLOCK: Item =
        { ItemT::from_block_stack_size(blocks::STRUCTURE_BLOCK.clone(), 64) };
} // GameMasterBlockItem<>
lazy_static! {
    pub static ref JIGSAW: Item = { ItemT::from_block_stack_size(blocks::JIGSAW.clone(), 64) };
} // GameMasterBlockItem<>
lazy_static! {
    pub static ref COMPOSTER: Item = { Arc::new(ItemT::from(blocks::COMPOSTER.clone())) };
}
lazy_static! {
    pub static ref TURTLE_HELMET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.TURTLE, EquipmentSlot.HEAD, >
lazy_static! {
    pub static ref SCUTE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref IRON_SHOVEL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ShovelItem<Tiers.IRON, 1.5F, -3.0F, >
lazy_static! {
    pub static ref IRON_PICKAXE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // PickaxeItem<Tiers.IRON, 1, -2.8F, >
lazy_static! {
    pub static ref IRON_AXE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // AxeItem<Tiers.IRON, 6.0F, -3.1F, >
lazy_static! {
    pub static ref FLINT_AND_STEEL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 64,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // FlintAndSteelItem<>
lazy_static! {
    pub static ref APPLE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 4,
                saturation_mod: 0.30,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref BOW: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 384,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BowItem<>
lazy_static! {
    pub static ref ARROW: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArrowItem<>
lazy_static! {
    pub static ref COAL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref CHARCOAL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref DIAMOND: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref IRON_INGOT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref GOLD_INGOT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref IRON_SWORD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SwordItem<Tiers.IRON, 3, -2.4F, >
lazy_static! {
    pub static ref WOODEN_SWORD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SwordItem<Tiers.WOOD, 3, -2.4F, >
lazy_static! {
    pub static ref WOODEN_SHOVEL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ShovelItem<Tiers.WOOD, 1.5F, -3.0F, >
lazy_static! {
    pub static ref WOODEN_PICKAXE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // PickaxeItem<Tiers.WOOD, 1, -2.8F, >
lazy_static! {
    pub static ref WOODEN_AXE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // AxeItem<Tiers.WOOD, 6.0F, -3.2F, >
lazy_static! {
    pub static ref STONE_SWORD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SwordItem<Tiers.STONE, 3, -2.4F, >
lazy_static! {
    pub static ref STONE_SHOVEL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ShovelItem<Tiers.STONE, 1.5F, -3.0F, >
lazy_static! {
    pub static ref STONE_PICKAXE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // PickaxeItem<Tiers.STONE, 1, -2.8F, >
lazy_static! {
    pub static ref STONE_AXE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // AxeItem<Tiers.STONE, 7.0F, -3.2F, >
lazy_static! {
    pub static ref DIAMOND_SWORD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SwordItem<Tiers.DIAMOND, 3, -2.4F, >
lazy_static! {
    pub static ref DIAMOND_SHOVEL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ShovelItem<Tiers.DIAMOND, 1.5F, -3.0F, >
lazy_static! {
    pub static ref DIAMOND_PICKAXE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // PickaxeItem<Tiers.DIAMOND, 1, -2.8F, >
lazy_static! {
    pub static ref DIAMOND_AXE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // AxeItem<Tiers.DIAMOND, 5.0F, -3.0F, >
lazy_static! {
    pub static ref STICK: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref BOWL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref MUSHROOM_STEW: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 6,
                saturation_mod: 0.60,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // BowlFoodItem<>
lazy_static! {
    pub static ref GOLDEN_SWORD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SwordItem<Tiers.GOLD, 3, -2.4F, >
lazy_static! {
    pub static ref GOLDEN_SHOVEL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ShovelItem<Tiers.GOLD, 1.5F, -3.0F, >
lazy_static! {
    pub static ref GOLDEN_PICKAXE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // PickaxeItem<Tiers.GOLD, 1, -2.8F, >
lazy_static! {
    pub static ref GOLDEN_AXE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // AxeItem<Tiers.GOLD, 6.0F, -3.0F, >
lazy_static! {
    pub static ref STRING: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ItemNameBlockItem<Blocks.TRIPWIRE, >
lazy_static! {
    pub static ref FEATHER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref GUNPOWDER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref WOODEN_HOE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // HoeItem<Tiers.WOOD, -3.0F, >
lazy_static! {
    pub static ref STONE_HOE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // HoeItem<Tiers.STONE, -2.0F, >
lazy_static! {
    pub static ref IRON_HOE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // HoeItem<Tiers.IRON, -1.0F, >
lazy_static! {
    pub static ref DIAMOND_HOE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // HoeItem<Tiers.DIAMOND, 0.0F, >
lazy_static! {
    pub static ref GOLDEN_HOE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // HoeItem<Tiers.GOLD, -3.0F, >
lazy_static! {
    pub static ref WHEAT_SEEDS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ItemNameBlockItem<Blocks.WHEAT, >
lazy_static! {
    pub static ref WHEAT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref BREAD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 5,
                saturation_mod: 0.60,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref LEATHER_HELMET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeableArmorItem<ArmorMaterials.LEATHER, EquipmentSlot.HEAD, >
lazy_static! {
    pub static ref LEATHER_CHESTPLATE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeableArmorItem<ArmorMaterials.LEATHER, EquipmentSlot.CHEST, >
lazy_static! {
    pub static ref LEATHER_LEGGINGS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeableArmorItem<ArmorMaterials.LEATHER, EquipmentSlot.LEGS, >
lazy_static! {
    pub static ref LEATHER_BOOTS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeableArmorItem<ArmorMaterials.LEATHER, EquipmentSlot.FEET, >
lazy_static! {
    pub static ref CHAINMAIL_HELMET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.CHAIN, EquipmentSlot.HEAD, >
lazy_static! {
    pub static ref CHAINMAIL_CHESTPLATE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.CHAIN, EquipmentSlot.CHEST, >
lazy_static! {
    pub static ref CHAINMAIL_LEGGINGS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.CHAIN, EquipmentSlot.LEGS, >
lazy_static! {
    pub static ref CHAINMAIL_BOOTS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.CHAIN, EquipmentSlot.FEET, >
lazy_static! {
    pub static ref IRON_HELMET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.IRON, EquipmentSlot.HEAD, >
lazy_static! {
    pub static ref IRON_CHESTPLATE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.IRON, EquipmentSlot.CHEST, >
lazy_static! {
    pub static ref IRON_LEGGINGS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.IRON, EquipmentSlot.LEGS, >
lazy_static! {
    pub static ref IRON_BOOTS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.IRON, EquipmentSlot.FEET, >
lazy_static! {
    pub static ref DIAMOND_HELMET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.DIAMOND, EquipmentSlot.HEAD, >
lazy_static! {
    pub static ref DIAMOND_CHESTPLATE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.DIAMOND, EquipmentSlot.CHEST, >
lazy_static! {
    pub static ref DIAMOND_LEGGINGS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.DIAMOND, EquipmentSlot.LEGS, >
lazy_static! {
    pub static ref DIAMOND_BOOTS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.DIAMOND, EquipmentSlot.FEET, >
lazy_static! {
    pub static ref GOLDEN_HELMET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.GOLD, EquipmentSlot.HEAD, >
lazy_static! {
    pub static ref GOLDEN_CHESTPLATE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.GOLD, EquipmentSlot.CHEST, >
lazy_static! {
    pub static ref GOLDEN_LEGGINGS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.GOLD, EquipmentSlot.LEGS, >
lazy_static! {
    pub static ref GOLDEN_BOOTS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorItem<ArmorMaterials.GOLD, EquipmentSlot.FEET, >
lazy_static! {
    pub static ref FLINT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref PORKCHOP: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 3,
                saturation_mod: 0.30,
                is_meat: true,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref COOKED_PORKCHOP: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 8,
                saturation_mod: 0.80,
                is_meat: true,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref PAINTING: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // HangingEntityItem<EntityType.PAINTING, >
lazy_static! {
    pub static ref GOLDEN_APPLE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 4,
                saturation_mod: 1.20,
                is_meat: false,
                can_always_eat: true,
                fast_food: false,
                effects: vec![
                    (mob_effects::REGENERATION.instance(100, 1), 1.00),
                    (mob_effects::ABSORPTION.instance(2400, 0), 1.00),
                ],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref ENCHANTED_GOLDEN_APPLE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 4,
                saturation_mod: 1.20,
                is_meat: false,
                can_always_eat: true,
                fast_food: false,
                effects: vec![
                    (mob_effects::REGENERATION.instance(400, 1), 1.00),
                    (mob_effects::DAMAGE_RESISTANCE.instance(6000, 0), 1.00),
                    (mob_effects::FIRE_RESISTANCE.instance(6000, 0), 1.00),
                    (mob_effects::ABSORPTION.instance(2400, 3), 1.00),
                ],
            }),
        })
    };
} // EnchantedGoldenAppleItem<>
lazy_static! {
    pub static ref OAK_SIGN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SignItem<>
lazy_static! {
    pub static ref SPRUCE_SIGN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SignItem<>
lazy_static! {
    pub static ref BIRCH_SIGN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SignItem<>
lazy_static! {
    pub static ref JUNGLE_SIGN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SignItem<>
lazy_static! {
    pub static ref ACACIA_SIGN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SignItem<>
lazy_static! {
    pub static ref DARK_OAK_SIGN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SignItem<>
lazy_static! {
    pub static ref BUCKET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BucketItem<Fluids.EMPTY, >
lazy_static! {
    pub static ref WATER_BUCKET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: Some(BUCKET.clone()),
            food_properties: None,
        })
    };
} // BucketItem<Fluids.WATER, >
lazy_static! {
    pub static ref LAVA_BUCKET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: Some(BUCKET.clone()),
            food_properties: None,
        })
    };
} // BucketItem<Fluids.LAVA, >
lazy_static! {
    pub static ref MINECART: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // MinecartItem<AbstractMinecart.Type.RIDEABLE, >
lazy_static! {
    pub static ref SADDLE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SaddleItem<>
lazy_static! {
    pub static ref REDSTONE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ItemNameBlockItem<Blocks.REDSTONE_WIRE, >
lazy_static! {
    pub static ref SNOWBALL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SnowballItem<>
lazy_static! {
    pub static ref OAK_BOAT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BoatItem<Boat.Type.OAK, >
lazy_static! {
    pub static ref LEATHER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref MILK_BUCKET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: Some(BUCKET.clone()),
            food_properties: None,
        })
    };
} // MilkBucketItem<>
lazy_static! {
    pub static ref PUFFERFISH_BUCKET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // FishBucketItem<EntityType.PUFFERFISH, Fluids.WATER, >
lazy_static! {
    pub static ref SALMON_BUCKET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // FishBucketItem<EntityType.SALMON, Fluids.WATER, >
lazy_static! {
    pub static ref COD_BUCKET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // FishBucketItem<EntityType.COD, Fluids.WATER, >
lazy_static! {
    pub static ref TROPICAL_FISH_BUCKET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // FishBucketItem<EntityType.TROPICAL_FISH, Fluids.WATER, >
lazy_static! {
    pub static ref BRICK: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref CLAY_BALL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref SUGAR_CANE: Item = { Arc::new(ItemT::from(blocks::SUGAR_CANE.clone())) };
}
lazy_static! {
    pub static ref KELP: Item = { Arc::new(ItemT::from(blocks::KELP.clone())) };
}
lazy_static! {
    pub static ref DRIED_KELP_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::DRIED_KELP_BLOCK.clone())) };
}
lazy_static! {
    pub static ref BAMBOO: Item = { Arc::new(ItemT::from(blocks::BAMBOO.clone())) };
}
lazy_static! {
    pub static ref PAPER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref BOOK: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BookItem<>
lazy_static! {
    pub static ref SLIME_BALL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref CHEST_MINECART: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // MinecartItem<AbstractMinecart.Type.CHEST, >
lazy_static! {
    pub static ref FURNACE_MINECART: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // MinecartItem<AbstractMinecart.Type.FURNACE, >
lazy_static! {
    pub static ref EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // EggItem<>
lazy_static! {
    pub static ref COMPASS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // CompassItem<>
lazy_static! {
    pub static ref FISHING_ROD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 64,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // FishingRodItem<>
lazy_static! {
    pub static ref CLOCK: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ClockItem<>
lazy_static! {
    pub static ref GLOWSTONE_DUST: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref COD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 2,
                saturation_mod: 0.10,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref SALMON: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 2,
                saturation_mod: 0.10,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref TROPICAL_FISH: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 1,
                saturation_mod: 0.10,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref PUFFERFISH: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 1,
                saturation_mod: 0.10,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![
                    (mob_effects::POISON.instance(1200, 3), 1.00),
                    (mob_effects::HUNGER.instance(300, 2), 1.00),
                    (mob_effects::CONFUSION.instance(300, 1), 1.00),
                ],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref COOKED_COD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 5,
                saturation_mod: 0.60,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref COOKED_SALMON: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 6,
                saturation_mod: 0.80,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref INK_SAC: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref RED_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.RED, >
lazy_static! {
    pub static ref GREEN_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.GREEN, >
lazy_static! {
    pub static ref COCOA_BEANS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ItemNameBlockItem<Blocks.COCOA, >
lazy_static! {
    pub static ref LAPIS_LAZULI: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref PURPLE_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.PURPLE, >
lazy_static! {
    pub static ref CYAN_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.CYAN, >
lazy_static! {
    pub static ref LIGHT_GRAY_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.LIGHT_GRAY, >
lazy_static! {
    pub static ref GRAY_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.GRAY, >
lazy_static! {
    pub static ref PINK_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.PINK, >
lazy_static! {
    pub static ref LIME_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.LIME, >
lazy_static! {
    pub static ref YELLOW_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.YELLOW, >
lazy_static! {
    pub static ref LIGHT_BLUE_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.LIGHT_BLUE, >
lazy_static! {
    pub static ref MAGENTA_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.MAGENTA, >
lazy_static! {
    pub static ref ORANGE_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.ORANGE, >
lazy_static! {
    pub static ref BONE_MEAL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BoneMealItem<>
lazy_static! {
    pub static ref BLUE_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.BLUE, >
lazy_static! {
    pub static ref BROWN_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.BROWN, >
lazy_static! {
    pub static ref BLACK_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.BLACK, >
lazy_static! {
    pub static ref WHITE_DYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeItem<DyeColor.WHITE, >
lazy_static! {
    pub static ref BONE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref SUGAR: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref CAKE: Item = { ItemT::from_block_stack_size(blocks::CAKE.clone(), 1) };
} // BlockItem<>
lazy_static! {
    pub static ref WHITE_BED: Item = { ItemT::from_block_stack_size(blocks::WHITE_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref ORANGE_BED: Item =
        { ItemT::from_block_stack_size(blocks::ORANGE_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref MAGENTA_BED: Item =
        { ItemT::from_block_stack_size(blocks::MAGENTA_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref LIGHT_BLUE_BED: Item =
        { ItemT::from_block_stack_size(blocks::LIGHT_BLUE_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref YELLOW_BED: Item =
        { ItemT::from_block_stack_size(blocks::YELLOW_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref LIME_BED: Item = { ItemT::from_block_stack_size(blocks::LIME_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref PINK_BED: Item = { ItemT::from_block_stack_size(blocks::PINK_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref GRAY_BED: Item = { ItemT::from_block_stack_size(blocks::GRAY_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref LIGHT_GRAY_BED: Item =
        { ItemT::from_block_stack_size(blocks::LIGHT_GRAY_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref CYAN_BED: Item = { ItemT::from_block_stack_size(blocks::CYAN_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref PURPLE_BED: Item =
        { ItemT::from_block_stack_size(blocks::PURPLE_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref BLUE_BED: Item = { ItemT::from_block_stack_size(blocks::BLUE_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref BROWN_BED: Item = { ItemT::from_block_stack_size(blocks::BROWN_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref GREEN_BED: Item = { ItemT::from_block_stack_size(blocks::GREEN_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref RED_BED: Item = { ItemT::from_block_stack_size(blocks::RED_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref BLACK_BED: Item = { ItemT::from_block_stack_size(blocks::BLACK_BED.clone(), 1) };
} // BedItem<>
lazy_static! {
    pub static ref COOKIE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 2,
                saturation_mod: 0.10,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref FILLED_MAP: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // MapItem<>
lazy_static! {
    pub static ref SHEARS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 238,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ShearsItem<>
lazy_static! {
    pub static ref MELON_SLICE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 2,
                saturation_mod: 0.30,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref DRIED_KELP: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 1,
                saturation_mod: 0.30,
                is_meat: false,
                can_always_eat: false,
                fast_food: true,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref PUMPKIN_SEEDS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ItemNameBlockItem<Blocks.PUMPKIN_STEM, >
lazy_static! {
    pub static ref MELON_SEEDS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ItemNameBlockItem<Blocks.MELON_STEM, >
lazy_static! {
    pub static ref BEEF: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 3,
                saturation_mod: 0.30,
                is_meat: true,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref COOKED_BEEF: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 8,
                saturation_mod: 0.80,
                is_meat: true,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref CHICKEN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 2,
                saturation_mod: 0.30,
                is_meat: true,
                can_always_eat: false,
                fast_food: false,
                effects: vec![(mob_effects::HUNGER.instance(600, 0), 0.30)],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref COOKED_CHICKEN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 6,
                saturation_mod: 0.60,
                is_meat: true,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref ROTTEN_FLESH: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 4,
                saturation_mod: 0.10,
                is_meat: true,
                can_always_eat: false,
                fast_food: false,
                effects: vec![(mob_effects::HUNGER.instance(600, 0), 0.80)],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref ENDER_PEARL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // EnderpearlItem<>
lazy_static! {
    pub static ref BLAZE_ROD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref GHAST_TEAR: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref GOLD_NUGGET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref NETHER_WART: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ItemNameBlockItem<Blocks.NETHER_WART, >
lazy_static! {
    pub static ref POTION: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // PotionItem<>
lazy_static! {
    pub static ref GLASS_BOTTLE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BottleItem<>
lazy_static! {
    pub static ref SPIDER_EYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 2,
                saturation_mod: 0.80,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![(mob_effects::POISON.instance(100, 0), 1.00)],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref FERMENTED_SPIDER_EYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref BLAZE_POWDER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref MAGMA_CREAM: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref BREWING_STAND: Item = { Arc::new(ItemT::from(blocks::BREWING_STAND.clone())) };
}
lazy_static! {
    pub static ref CAULDRON: Item = { Arc::new(ItemT::from(blocks::CAULDRON.clone())) };
}
lazy_static! {
    pub static ref ENDER_EYE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // EnderEyeItem<>
lazy_static! {
    pub static ref GLISTERING_MELON_SLICE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref BAT_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.BAT, 4996656, 986895, >
lazy_static! {
    pub static ref BEE_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.BEE, 15582019, 4400155, >
lazy_static! {
    pub static ref BLAZE_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.BLAZE, 16167425, 16775294, >
lazy_static! {
    pub static ref CAT_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.CAT, 15714446, 9794134, >
lazy_static! {
    pub static ref CAVE_SPIDER_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.CAVE_SPIDER, 803406, 11013646, >
lazy_static! {
    pub static ref CHICKEN_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.CHICKEN, 10592673, 16711680, >
lazy_static! {
    pub static ref COD_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.COD, 12691306, 15058059, >
lazy_static! {
    pub static ref COW_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.COW, 4470310, 10592673, >
lazy_static! {
    pub static ref CREEPER_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.CREEPER, 894731, 0, >
lazy_static! {
    pub static ref DOLPHIN_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.DOLPHIN, 2243405, 16382457, >
lazy_static! {
    pub static ref DONKEY_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.DONKEY, 5457209, 8811878, >
lazy_static! {
    pub static ref DROWNED_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.DROWNED, 9433559, 7969893, >
lazy_static! {
    pub static ref ELDER_GUARDIAN_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.ELDER_GUARDIAN, 13552826, 7632531, >
lazy_static! {
    pub static ref ENDERMAN_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.ENDERMAN, 1447446, 0, >
lazy_static! {
    pub static ref ENDERMITE_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.ENDERMITE, 1447446, 7237230, >
lazy_static! {
    pub static ref EVOKER_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.EVOKER, 9804699, 1973274, >
lazy_static! {
    pub static ref FOX_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.FOX, 14005919, 13396256, >
lazy_static! {
    pub static ref GHAST_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.GHAST, 16382457, 12369084, >
lazy_static! {
    pub static ref GUARDIAN_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.GUARDIAN, 5931634, 15826224, >
lazy_static! {
    pub static ref HORSE_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.HORSE, 12623485, 15656192, >
lazy_static! {
    pub static ref HUSK_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.HUSK, 7958625, 15125652, >
lazy_static! {
    pub static ref LLAMA_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.LLAMA, 12623485, 10051392, >
lazy_static! {
    pub static ref MAGMA_CUBE_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.MAGMA_CUBE, 3407872, 16579584, >
lazy_static! {
    pub static ref MOOSHROOM_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.MOOSHROOM, 10489616, 12040119, >
lazy_static! {
    pub static ref MULE_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.MULE, 1769984, 5321501, >
lazy_static! {
    pub static ref OCELOT_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.OCELOT, 15720061, 5653556, >
lazy_static! {
    pub static ref PANDA_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.PANDA, 15198183, 1776418, >
lazy_static! {
    pub static ref PARROT_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.PARROT, 894731, 16711680, >
lazy_static! {
    pub static ref PHANTOM_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.PHANTOM, 4411786, 8978176, >
lazy_static! {
    pub static ref PIG_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.PIG, 15771042, 14377823, >
lazy_static! {
    pub static ref PILLAGER_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.PILLAGER, 5451574, 9804699, >
lazy_static! {
    pub static ref POLAR_BEAR_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.POLAR_BEAR, 15921906, 9803152, >
lazy_static! {
    pub static ref PUFFERFISH_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.PUFFERFISH, 16167425, 3654642, >
lazy_static! {
    pub static ref RABBIT_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.RABBIT, 10051392, 7555121, >
lazy_static! {
    pub static ref RAVAGER_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.RAVAGER, 7697520, 5984329, >
lazy_static! {
    pub static ref SALMON_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.SALMON, 10489616, 951412, >
lazy_static! {
    pub static ref SHEEP_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.SHEEP, 15198183, 16758197, >
lazy_static! {
    pub static ref SHULKER_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.SHULKER, 9725844, 5060690, >
lazy_static! {
    pub static ref SILVERFISH_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.SILVERFISH, 7237230, 3158064, >
lazy_static! {
    pub static ref SKELETON_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.SKELETON, 12698049, 4802889, >
lazy_static! {
    pub static ref SKELETON_HORSE_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.SKELETON_HORSE, 6842447, 15066584, >
lazy_static! {
    pub static ref SLIME_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.SLIME, 5349438, 8306542, >
lazy_static! {
    pub static ref SPIDER_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.SPIDER, 3419431, 11013646, >
lazy_static! {
    pub static ref SQUID_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.SQUID, 2243405, 7375001, >
lazy_static! {
    pub static ref STRAY_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.STRAY, 6387319, 14543594, >
lazy_static! {
    pub static ref TRADER_LLAMA_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.TRADER_LLAMA, 15377456, 4547222, >
lazy_static! {
    pub static ref TROPICAL_FISH_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.TROPICAL_FISH, 15690005, 16775663, >
lazy_static! {
    pub static ref TURTLE_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.TURTLE, 15198183, 44975, >
lazy_static! {
    pub static ref VEX_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.VEX, 8032420, 15265265, >
lazy_static! {
    pub static ref VILLAGER_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.VILLAGER, 5651507, 12422002, >
lazy_static! {
    pub static ref VINDICATOR_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.VINDICATOR, 9804699, 2580065, >
lazy_static! {
    pub static ref WANDERING_TRADER_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.WANDERING_TRADER, 4547222, 15377456, >
lazy_static! {
    pub static ref WITCH_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.WITCH, 3407872, 5349438, >
lazy_static! {
    pub static ref WITHER_SKELETON_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.WITHER_SKELETON, 1315860, 4672845, >
lazy_static! {
    pub static ref WOLF_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.WOLF, 14144467, 13545366, >
lazy_static! {
    pub static ref ZOMBIE_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.ZOMBIE, 44975, 7969893, >
lazy_static! {
    pub static ref ZOMBIE_HORSE_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.ZOMBIE_HORSE, 3232308, 9945732, >
lazy_static! {
    pub static ref ZOMBIE_PIGMAN_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.ZOMBIE_PIGMAN, 15373203, 5009705, >
lazy_static! {
    pub static ref ZOMBIE_VILLAGER_SPAWN_EGG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpawnEggItem<EntityType.ZOMBIE_VILLAGER, 5651507, 7969893, >
lazy_static! {
    pub static ref EXPERIENCE_BOTTLE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ExperienceBottleItem<>
lazy_static! {
    pub static ref FIRE_CHARGE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // FireChargeItem<>
lazy_static! {
    pub static ref WRITABLE_BOOK: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // WritableBookItem<>
lazy_static! {
    pub static ref WRITTEN_BOOK: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // WrittenBookItem<>
lazy_static! {
    pub static ref EMERALD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref ITEM_FRAME: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ItemFrameItem<>
lazy_static! {
    pub static ref FLOWER_POT: Item = { Arc::new(ItemT::from(blocks::FLOWER_POT.clone())) };
}
lazy_static! {
    pub static ref CARROT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 3,
                saturation_mod: 0.60,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // ItemNameBlockItem<Blocks.CARROTS, >
lazy_static! {
    pub static ref POTATO: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 1,
                saturation_mod: 0.30,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // ItemNameBlockItem<Blocks.POTATOES, >
lazy_static! {
    pub static ref BAKED_POTATO: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 5,
                saturation_mod: 0.60,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref POISONOUS_POTATO: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 2,
                saturation_mod: 0.30,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![(mob_effects::POISON.instance(100, 0), 0.60)],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref MAP: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // EmptyMapItem<>
lazy_static! {
    pub static ref GOLDEN_CARROT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 6,
                saturation_mod: 1.20,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref SKELETON_SKULL: Item =
        { ItemT::from_block_stack_size(blocks::SKELETON_SKULL.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.SKELETON_WALL_SKULL>
lazy_static! {
    pub static ref WITHER_SKELETON_SKULL: Item =
        { ItemT::from_block_stack_size(blocks::WITHER_SKELETON_SKULL.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.WITHER_SKELETON_WALL_SKULL>
lazy_static! {
    pub static ref PLAYER_HEAD: Item =
        { ItemT::from_block_stack_size(blocks::PLAYER_HEAD.clone(), 64) };
} // PlayerHeadItem<, Blocks.PLAYER_WALL_HEAD>
lazy_static! {
    pub static ref ZOMBIE_HEAD: Item =
        { ItemT::from_block_stack_size(blocks::ZOMBIE_HEAD.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.ZOMBIE_WALL_HEAD>
lazy_static! {
    pub static ref CREEPER_HEAD: Item =
        { ItemT::from_block_stack_size(blocks::CREEPER_HEAD.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.CREEPER_WALL_HEAD>
lazy_static! {
    pub static ref DRAGON_HEAD: Item =
        { ItemT::from_block_stack_size(blocks::DRAGON_HEAD.clone(), 64) };
} // StandingAndWallBlockItem<, Blocks.DRAGON_WALL_HEAD>
lazy_static! {
    pub static ref CARROT_ON_A_STICK: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 25,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // CarrotOnAStickItem<>
lazy_static! {
    pub static ref NETHER_STAR: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SimpleFoiledItem<>
lazy_static! {
    pub static ref PUMPKIN_PIE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 8,
                saturation_mod: 0.30,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref FIREWORK_ROCKET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // FireworkRocketItem<>
lazy_static! {
    pub static ref FIREWORK_STAR: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // FireworkStarItem<>
lazy_static! {
    pub static ref ENCHANTED_BOOK: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // EnchantedBookItem<>
lazy_static! {
    pub static ref NETHER_BRICK: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref QUARTZ: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref TNT_MINECART: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // MinecartItem<AbstractMinecart.Type.TNT, >
lazy_static! {
    pub static ref HOPPER_MINECART: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // MinecartItem<AbstractMinecart.Type.HOPPER, >
lazy_static! {
    pub static ref PRISMARINE_SHARD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref PRISMARINE_CRYSTALS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref RABBIT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 3,
                saturation_mod: 0.30,
                is_meat: true,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref COOKED_RABBIT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 5,
                saturation_mod: 0.60,
                is_meat: true,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref RABBIT_STEW: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 10,
                saturation_mod: 0.60,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // BowlFoodItem<>
lazy_static! {
    pub static ref RABBIT_FOOT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref RABBIT_HIDE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref ARMOR_STAND: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ArmorStandItem<>
lazy_static! {
    pub static ref IRON_HORSE_ARMOR: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // HorseArmorItem<5, "iron", >
lazy_static! {
    pub static ref GOLDEN_HORSE_ARMOR: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // HorseArmorItem<7, "gold", >
lazy_static! {
    pub static ref DIAMOND_HORSE_ARMOR: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // HorseArmorItem<11, "diamond", >
lazy_static! {
    pub static ref LEATHER_HORSE_ARMOR: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DyeableHorseArmorItem<3, "leather", >
lazy_static! {
    pub static ref LEAD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // LeadItem<>
lazy_static! {
    pub static ref NAME_TAG: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // NameTagItem<>
lazy_static! {
    pub static ref COMMAND_BLOCK_MINECART: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // MinecartItem<AbstractMinecart.Type.COMMAND_BLOCK, >
lazy_static! {
    pub static ref MUTTON: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 2,
                saturation_mod: 0.30,
                is_meat: true,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref COOKED_MUTTON: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 6,
                saturation_mod: 0.80,
                is_meat: true,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref WHITE_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.WHITE_BANNER, Blocks.WHITE_WALL_BANNER, >
lazy_static! {
    pub static ref ORANGE_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.ORANGE_BANNER, Blocks.ORANGE_WALL_BANNER, >
lazy_static! {
    pub static ref MAGENTA_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.MAGENTA_BANNER, Blocks.MAGENTA_WALL_BANNER, >
lazy_static! {
    pub static ref LIGHT_BLUE_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.LIGHT_BLUE_BANNER, Blocks.LIGHT_BLUE_WALL_BANNER, >
lazy_static! {
    pub static ref YELLOW_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.YELLOW_BANNER, Blocks.YELLOW_WALL_BANNER, >
lazy_static! {
    pub static ref LIME_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.LIME_BANNER, Blocks.LIME_WALL_BANNER, >
lazy_static! {
    pub static ref PINK_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.PINK_BANNER, Blocks.PINK_WALL_BANNER, >
lazy_static! {
    pub static ref GRAY_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.GRAY_BANNER, Blocks.GRAY_WALL_BANNER, >
lazy_static! {
    pub static ref LIGHT_GRAY_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.LIGHT_GRAY_BANNER, Blocks.LIGHT_GRAY_WALL_BANNER, >
lazy_static! {
    pub static ref CYAN_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.CYAN_BANNER, Blocks.CYAN_WALL_BANNER, >
lazy_static! {
    pub static ref PURPLE_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.PURPLE_BANNER, Blocks.PURPLE_WALL_BANNER, >
lazy_static! {
    pub static ref BLUE_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.BLUE_BANNER, Blocks.BLUE_WALL_BANNER, >
lazy_static! {
    pub static ref BROWN_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.BROWN_BANNER, Blocks.BROWN_WALL_BANNER, >
lazy_static! {
    pub static ref GREEN_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.GREEN_BANNER, Blocks.GREEN_WALL_BANNER, >
lazy_static! {
    pub static ref RED_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.RED_BANNER, Blocks.RED_WALL_BANNER, >
lazy_static! {
    pub static ref BLACK_BANNER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerItem<Blocks.BLACK_BANNER, Blocks.BLACK_WALL_BANNER, >
lazy_static! {
    pub static ref END_CRYSTAL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // EndCrystalItem<>
lazy_static! {
    pub static ref CHORUS_FRUIT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 4,
                saturation_mod: 0.30,
                is_meat: false,
                can_always_eat: true,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // ChorusFruitItem<>
lazy_static! {
    pub static ref POPPED_CHORUS_FRUIT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref BEETROOT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 1,
                saturation_mod: 0.60,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // Item<>
lazy_static! {
    pub static ref BEETROOT_SEEDS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ItemNameBlockItem<Blocks.BEETROOTS, >
lazy_static! {
    pub static ref BEETROOT_SOUP: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 6,
                saturation_mod: 0.60,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // BowlFoodItem<>
lazy_static! {
    pub static ref DRAGON_BREATH: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: Some(GLASS_BOTTLE.clone()),
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref SPLASH_POTION: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SplashPotionItem<>
lazy_static! {
    pub static ref SPECTRAL_ARROW: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // SpectralArrowItem<>
lazy_static! {
    pub static ref TIPPED_ARROW: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // TippedArrowItem<>
lazy_static! {
    pub static ref LINGERING_POTION: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // LingeringPotionItem<>
lazy_static! {
    pub static ref SHIELD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 336,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ShieldItem<>
lazy_static! {
    pub static ref ELYTRA: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 432,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // ElytraItem<>
lazy_static! {
    pub static ref SPRUCE_BOAT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BoatItem<Boat.Type.SPRUCE, >
lazy_static! {
    pub static ref BIRCH_BOAT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BoatItem<Boat.Type.BIRCH, >
lazy_static! {
    pub static ref JUNGLE_BOAT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BoatItem<Boat.Type.JUNGLE, >
lazy_static! {
    pub static ref ACACIA_BOAT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BoatItem<Boat.Type.ACACIA, >
lazy_static! {
    pub static ref DARK_OAK_BOAT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BoatItem<Boat.Type.DARK_OAK, >
lazy_static! {
    pub static ref TOTEM_OF_UNDYING: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref SHULKER_SHELL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref IRON_NUGGET: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref KNOWLEDGE_BOOK: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // KnowledgeBookItem<>
lazy_static! {
    pub static ref DEBUG_STICK: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // DebugStickItem<>
lazy_static! {
    pub static ref MUSIC_DISC_13: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<1, SoundEvents.MUSIC_DISC_13, >
lazy_static! {
    pub static ref MUSIC_DISC_CAT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<2, SoundEvents.MUSIC_DISC_CAT, >
lazy_static! {
    pub static ref MUSIC_DISC_BLOCKS: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<3, SoundEvents.MUSIC_DISC_BLOCKS, >
lazy_static! {
    pub static ref MUSIC_DISC_CHIRP: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<4, SoundEvents.MUSIC_DISC_CHIRP, >
lazy_static! {
    pub static ref MUSIC_DISC_FAR: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<5, SoundEvents.MUSIC_DISC_FAR, >
lazy_static! {
    pub static ref MUSIC_DISC_MALL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<6, SoundEvents.MUSIC_DISC_MALL, >
lazy_static! {
    pub static ref MUSIC_DISC_MELLOHI: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<7, SoundEvents.MUSIC_DISC_MELLOHI, >
lazy_static! {
    pub static ref MUSIC_DISC_STAL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<8, SoundEvents.MUSIC_DISC_STAL, >
lazy_static! {
    pub static ref MUSIC_DISC_STRAD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<9, SoundEvents.MUSIC_DISC_STRAD, >
lazy_static! {
    pub static ref MUSIC_DISC_WARD: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<10, SoundEvents.MUSIC_DISC_WARD, >
lazy_static! {
    pub static ref MUSIC_DISC_11: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<11, SoundEvents.MUSIC_DISC_11, >
lazy_static! {
    pub static ref MUSIC_DISC_WAIT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // RecordItem<12, SoundEvents.MUSIC_DISC_WAIT, >
lazy_static! {
    pub static ref TRIDENT: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 250,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // TridentItem<>
lazy_static! {
    pub static ref PHANTOM_MEMBRANE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref NAUTILUS_SHELL: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref HEART_OF_THE_SEA: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref CROSSBOW: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 326,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // CrossbowItem<>
lazy_static! {
    pub static ref SUSPICIOUS_STEW: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 6,
                saturation_mod: 0.60,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // SuspiciousStewItem<>
lazy_static! {
    pub static ref LOOM: Item = { Arc::new(ItemT::from(blocks::LOOM.clone())) };
}
lazy_static! {
    pub static ref FLOWER_BANNER_PATTERN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerPatternItem<BannerPattern.FLOWER, >
lazy_static! {
    pub static ref CREEPER_BANNER_PATTERN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerPatternItem<BannerPattern.CREEPER, >
lazy_static! {
    pub static ref SKULL_BANNER_PATTERN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerPatternItem<BannerPattern.SKULL, >
lazy_static! {
    pub static ref MOJANG_BANNER_PATTERN: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerPatternItem<BannerPattern.MOJANG, >
lazy_static! {
    pub static ref GLOBE_BANNER_PATTER: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 1,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // BannerPatternItem<BannerPattern.GLOBE, >
lazy_static! {
    pub static ref BARREL: Item = { Arc::new(ItemT::from(blocks::BARREL.clone())) };
}
lazy_static! {
    pub static ref SMOKER: Item = { Arc::new(ItemT::from(blocks::SMOKER.clone())) };
}
lazy_static! {
    pub static ref BLAST_FURNACE: Item = { Arc::new(ItemT::from(blocks::BLAST_FURNACE.clone())) };
}
lazy_static! {
    pub static ref CARTOGRAPHY_TABLE: Item =
        { Arc::new(ItemT::from(blocks::CARTOGRAPHY_TABLE.clone())) };
}
lazy_static! {
    pub static ref FLETCHING_TABLE: Item =
        { Arc::new(ItemT::from(blocks::FLETCHING_TABLE.clone())) };
}
lazy_static! {
    pub static ref GRINDSTONE: Item = { Arc::new(ItemT::from(blocks::GRINDSTONE.clone())) };
}
lazy_static! {
    pub static ref LECTERN: Item = { Arc::new(ItemT::from(blocks::LECTERN.clone())) };
}
lazy_static! {
    pub static ref SMITHING_TABLE: Item = { Arc::new(ItemT::from(blocks::SMITHING_TABLE.clone())) };
}
lazy_static! {
    pub static ref STONECUTTER: Item = { Arc::new(ItemT::from(blocks::STONECUTTER.clone())) };
}
lazy_static! {
    pub static ref BELL: Item = { Arc::new(ItemT::from(blocks::BELL.clone())) };
}
lazy_static! {
    pub static ref LANTERN: Item = { Arc::new(ItemT::from(blocks::LANTERN.clone())) };
}
lazy_static! {
    pub static ref SWEET_BERRIES: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: Some(FoodProperties {
                nutrition: 2,
                saturation_mod: 0.10,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // ItemNameBlockItem<Blocks.SWEET_BERRY_BUSH, >
lazy_static! {
    pub static ref CAMPFIRE: Item = { Arc::new(ItemT::from(blocks::CAMPFIRE.clone())) };
}
lazy_static! {
    pub static ref HONEYCOMB: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    };
} // Item<>
lazy_static! {
    pub static ref BEE_NEST: Item = { Arc::new(ItemT::from(blocks::BEE_NEST.clone())) };
}
lazy_static! {
    pub static ref BEEHIVE: Item = { Arc::new(ItemT::from(blocks::BEEHIVE.clone())) };
}
lazy_static! {
    pub static ref HONEY_BOTTLE: Item = {
        Arc::new(ItemT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 16,
            max_damage: 0,
            crafting_remaining_item: Some(GLASS_BOTTLE.clone()),
            food_properties: Some(FoodProperties {
                nutrition: 6,
                saturation_mod: 0.10,
                is_meat: false,
                can_always_eat: false,
                fast_food: false,
                effects: vec![],
            }),
        })
    };
} // HoneyBottleItem<>
lazy_static! {
    pub static ref HONEY_BLOCK: Item = { Arc::new(ItemT::from(blocks::HONEY_BLOCK.clone())) };
}
lazy_static! {
    pub static ref HONEYCOMB_BLOCK: Item =
        { Arc::new(ItemT::from(blocks::HONEYCOMB_BLOCK.clone())) };
}
