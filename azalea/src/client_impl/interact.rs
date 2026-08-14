use azalea_client::interact::{
    EntityInteractEvent, StartUseItemEvent, StopUseItemEvent, pick::HitResultComponent,
};
use azalea_core::{hit_result::HitResult, position::BlockPos};
use azalea_protocol::packets::game::s_interact::InteractionHand;
use bevy_ecs::entity::Entity;

use crate::{Client, client_impl::error::AzaleaResult};

impl Client {
    /// Returns the current [`HitResult`], which is the block or entity in the
    /// client's crosshair.
    pub fn hit_result(&self) -> AzaleaResult<HitResult> {
        Ok((**self.component::<HitResultComponent>()?).clone())
    }

    /// Right-click a block.
    ///
    /// The behavior of this depends on the target block,
    /// and it'll either place the block you're holding in your hand or use the
    /// block you clicked (like toggling a lever).
    ///
    /// Note that this may trigger anticheats as it doesn't take into account
    /// whether you're actually looking at the block.
    pub fn block_interact(&self, position: BlockPos) {
        self.ecs.write().write_message(StartUseItemEvent {
            entity: self.entity,
            hand: InteractionHand::MainHand,
            force_block: Some(position),
        });
    }

    /// Right-click an entity.
    ///
    /// This can click through walls, which may trigger anticheats. If that
    /// behavior isn't desired, consider using [`Client::start_use_item`]
    /// instead.
    pub fn entity_interact(&self, entity: Entity) {
        self.ecs.write().trigger(EntityInteractEvent {
            client: self.entity,
            target: entity,
            location: None,
        });
    }

    /// Right-click the currently held item.
    ///
    /// If the item is consumable, then it'll act as if right-click was held
    /// until the item finishes being consumed. You can use this to eat food.
    ///
    /// If we're looking at a block or entity, then it will be clicked. Also see
    /// [`Client::block_interact`] and [`Client::entity_interact`].
    pub fn start_use_item(&self) {
        self.ecs.write().write_message(StartUseItemEvent {
            entity: self.entity,
            hand: InteractionHand::MainHand,
            force_block: None,
            force_miss: false,
        });
    }

    /// Right-click the currently held item, always sending `ServerboundUseItem`
    /// (right-click air) regardless of what we're looking at.
    ///
    /// This is for consuming food / drinking potions where we want the "use
    /// item on air" path even when the bot is looking at a block (e.g. inside
    /// caves). The regular [`Client::start_use_item`] would send
    /// `ServerboundUseItemOn` (right-click the block) when looking at a block,
    /// which the server rejects for consumables.
    pub fn use_item_air(&self) {
        self.ecs.write().write_message(StartUseItemEvent {
            entity: self.entity,
            hand: InteractionHand::MainHand,
            force_block: None,
            force_miss: true,
        });
    }

    /// Stop using the currently held item (simulate releasing the right-click).
    ///
    /// This is how you release an arrow from a bow, stop eating, etc. It sends
    /// a `ServerboundPlayerAction` with [`Action::ReleaseUseItem`].
    ///
    /// [`Action::ReleaseUseItem`]: azalea_protocol::packets::game::s_player_action::Action::ReleaseUseItem
    pub fn stop_use_item(&self) {
        self.ecs.write().write_message(StopUseItemEvent {
            entity: self.entity,
        });
    }
}
