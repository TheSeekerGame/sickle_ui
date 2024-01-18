use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::ui_builder::*;

#[derive(Debug, Default)]
pub struct RowConfig {
    pub height: Val,
    pub background_color: Color,
}

pub trait UiRowExt<'w, 's> {
    fn row<'a>(
        &'a mut self,
        config: RowConfig,
        f: impl FnOnce(&mut UiBuilder),
    ) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> UiRowExt<'w, 's> for UiBuilder<'w, 's, '_> {
    fn row<'a>(
        &'a mut self,
        config: RowConfig,
        spawn_children: impl FnOnce(&mut UiBuilder),
    ) -> EntityCommands<'w, 's, 'a> {
        let mut new_parent = Entity::PLACEHOLDER;

        if let Some(entity) = self.entity() {
            self.commands().entity(entity).with_children(|parent| {
                new_parent = parent.spawn(row_bundle(config)).id();
            });
        } else {
            new_parent = self.commands().spawn(row_bundle(config)).id();
        }

        let mut new_entity = self.commands().entity(new_parent);
        let mut new_builder = new_entity.ui_builder();
        spawn_children(&mut new_builder);

        self.commands().entity(new_parent)
    }
}

fn row_bundle(config: RowConfig) -> impl Bundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: config.height,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: config.background_color.into(),
        ..default()
    }
}