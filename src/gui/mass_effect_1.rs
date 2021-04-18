use async_recursion::async_recursion;
use imgui::*;
use std::cmp::Ordering;

use crate::save_data::{
    common::plot::*,
    mass_effect_1::{data::*, known_plot::*, player::*, *},
    SaveData,
};

use super::Gui;

impl<'a> Gui<'a> {
    pub async fn draw_mass_effect_1(&self, save_game: &mut Me1SaveGame) {
        let ui = self.ui;

        // TODO: Change ça
        let string = include_str!("../../plot/Me1KnownPlot.ron");
        let me1_known_plot: Me1KnownPlot = ron::from_str(&string).unwrap();

        // Tabs
        if let Some(_t) = TabBar::new(im_str!("mass_effect_1")).begin(ui) {
            // Plot
            if let Some(_t) = TabItem::new(im_str!("Plot")).begin(ui) {
                let me1_plot_table = &mut save_game.state.plot;
                if let Some(_t) = TabBar::new(im_str!("plot-tab")).begin(ui) {
                    if let Some(_t) = TabItem::new(im_str!("Player / Squad")).begin(ui) {
                        for (category_name, known_plot) in &me1_known_plot.player_squad {
                            if let Some(_t) = TreeNode::new(&ImString::new(category_name)).push(ui)
                            {
                                self.draw_known_plot(me1_plot_table, known_plot).await;
                            }
                        }
                    }
                    if let Some(_t) = TabItem::new(im_str!("Missions")).begin(ui) {
                        for (category_name, known_plot) in &me1_known_plot.missions {
                            if let Some(_t) = TreeNode::new(&ImString::new(category_name)).push(ui)
                            {
                                self.draw_known_plot(me1_plot_table, known_plot).await;
                            }
                        }
                    }
                }
            }
            // Raw
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui) {
                if let Some(_t) = TreeNode::new(im_str!("Mass Effect 1")).push(ui) {
                    // Player
                    self.draw_raw_player(&save_game.player).await;
                    // State
                    save_game.state.draw_raw_ui(self, "State").await;
                }
            }
        }
    }

    async fn draw_known_plot(&self, me1_plot_table: &mut Me1PlotTable, known_plot: &KnownPlot) {
        // Booleans
        for (plot_id, plot_desc) in &known_plot.booleans {
            let plot = me1_plot_table.bool_variables.get_mut(*plot_id);
            if let Some(mut plot) = plot {
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
        // Integers
        for (plot_id, plot_desc) in &known_plot.ints {
            let plot = me1_plot_table.int_variables.get_mut(*plot_id);
            if let Some(plot) = plot {
                plot.draw_raw_ui(self, plot_desc).await;
            }
        }
    }

    async fn draw_raw_player(&self, player: &Player) {
        for (i, _) in player.objects.iter().enumerate() {
            let object_id = i as i32 + 1;
            let object = player.get_object(object_id);
            let object_name = player.get_name(object.object_name_id);

            match object_name.to_str() {
                "CurrentGame" => self.draw_object(player, i, None, object_id).await,
                _ => continue,
            }
        }
    }

    async fn draw_object(
        &self, player: &Player, ident: usize, property_name: Option<&ImStr>, object_id: i32,
    ) {
        let object = player.get_object(object_id);
        let object_name = player.get_name(object.object_name_id);

        let property_name = match property_name {
            Option::Some(property_name) => im_str!("{} : {}", object_name, property_name),
            Option::None => object_name.to_owned(),
        };

        if let Some(_t) =
            TreeNode::new(&im_str!("object-{}", ident)).label(&property_name).push(self.ui)
        {
            let mut data = player.get_data(object_id).borrow_mut();
            for (i, property) in data.iter_mut().enumerate() {
                self.draw_property(player, i, property).await;
            }
        }
    }

    #[async_recursion(?Send)]
    async fn draw_property(&self, player: &Player, ident: usize, property: &mut Property) {
        match property {
            Property::Array { name_id, array, .. } => {
                self.draw_array_property(player, ident, player.get_name(*name_id), array).await
            }
            Property::Bool { name_id, value, .. } => {
                self.draw_edit_bool(
                    im_str!("{}##bool-{}", player.get_name(*name_id), ident).to_str(),
                    value,
                )
                .await
            }
            Property::Float { name_id, value, .. } => {
                self.draw_edit_f32(
                    im_str!("{}##f32-{}", player.get_name(*name_id), ident).to_str(),
                    value,
                )
                .await
            }
            Property::Int { name_id, value, .. } => {
                self.draw_edit_i32(
                    im_str!("{}##i32-{}", player.get_name(*name_id), ident).to_str(),
                    value,
                )
                .await
            }
            Property::Name { name_id, value_name_id, .. } => {
                self.draw_text(
                    player.get_name(*value_name_id),
                    Some(player.get_name(*name_id)),
                    ident,
                )
                .await;
            }
            Property::Object { name_id, object_id, .. } => {
                match (*object_id).cmp(&0) {
                    Ordering::Greater => {
                        // Object
                        let property_name = player.get_name(*name_id);
                        self.draw_object(player, ident, Some(property_name), *object_id).await;
                    }
                    Ordering::Less => {
                        // Class
                        let class = player.get_class(*object_id);
                        let class_name = player.get_name(class.class_name_id);
                        self.draw_text(class_name, Some(player.get_name(*name_id)), ident).await;
                    }
                    Ordering::Equal => {
                        // Null => Nom de classe par défaut
                        self.draw_text(&im_str!("Class"), Some(player.get_name(*name_id)), ident)
                            .await
                    }
                };
            }
            Property::Str { name_id, string, .. } => {
                self.draw_edit_string(
                    im_str!("{}##string-{}", player.get_name(*name_id), ident).to_str(),
                    string,
                )
                .await
            }
            Property::StringRef { name_id, value, .. } => {
                self.draw_edit_i32(
                    im_str!("{}##string-ref-{}", player.get_name(*name_id), ident).to_str(),
                    value,
                )
                .await;
            }
            Property::Struct { name_id, struct_name_id, properties, .. } => {
                self.draw_struct_property(
                    player,
                    ident,
                    &im_str!(
                        "{} : {}",
                        player.get_name(*struct_name_id),
                        player.get_name(*name_id),
                    ),
                    properties,
                )
                .await
            }
            _ => {}
        }
    }

    async fn draw_array_property(
        &self, player: &Player, ident: usize, label: &ImStr, array: &mut [ArrayType],
    ) {
        let ui = self.ui;
        if let Some(_t) = TreeNode::new(&im_str!("property-{}", ident))
            .label(&im_str!("Array : {}", label))
            .push(ui)
        {
            if !array.is_empty() {
                for (i, property) in array.iter_mut().enumerate() {
                    match property {
                        ArrayType::Int(int) => {
                            int.draw_raw_ui(self, im_str!("{}##int-{}", i, i).to_str()).await
                        }
                        ArrayType::Object(object_id) => {
                            if *object_id != 0 {
                                // Object
                                self.draw_object(player, i, None, *object_id).await;
                            } else {
                                // Null
                                self.draw_text(&im_str!("Null"), None, i).await;
                            }
                        }
                        ArrayType::Vector(vector) => {
                            vector.draw_raw_ui(self, im_str!("{}##vector-{}", i, i).to_str()).await
                        }
                        ArrayType::String(string) => {
                            string.draw_raw_ui(self, im_str!("##string-{}", i).to_str()).await
                        }
                        ArrayType::Properties(properties) => {
                            if let Some(_t) = TreeNode::new(&im_str!("array-properties-{}", i))
                                .label(&im_str!("{}", i))
                                .push(ui)
                            {
                                for (j, property) in properties.iter_mut().enumerate() {
                                    self.draw_property(player, j, property).await;
                                }
                            }
                        }
                    }
                }
            } else {
                self.draw_colored_bg("empty", || {
                    ui.align_text_to_frame_padding();
                    ui.text(" Empty");
                });
            }
        }
    }

    async fn draw_struct_property(
        &self, player: &Player, ident: usize, label: &ImStr, struct_property: &mut StructType,
    ) {
        match struct_property {
            StructType::LinearColor(color) => {
                color.draw_raw_ui(self, im_str!("{}##linear-color-{}", label, ident).to_str()).await
            }
            StructType::Vector(vector) => {
                vector.draw_raw_ui(self, im_str!("{}##vector-{}", label, ident).to_str()).await
            }
            StructType::Rotator(rotator) => {
                rotator.draw_raw_ui(self, im_str!("{}##rotator-{}", label, ident).to_str()).await
            }
            StructType::Properties(properties) => {
                if let Some(_t) = TreeNode::new(&im_str!("struct-properties-{}", ident))
                    .label(label)
                    .push(self.ui)
                {
                    for (i, property) in properties.iter_mut().enumerate() {
                        self.draw_property(player, i, property).await;
                    }
                }
            }
        }
    }

    async fn draw_text(&self, text: &ImStr, label: Option<&ImStr>, ident: usize) {
        let ui = self.ui;
        self.draw_colored_bg(im_str!("##text-{}", ident).to_str(), || {
            ui.align_text_to_frame_padding();
            ui.text(im_str!(" {}", text));

            if let Some(label) = label {
                ui.same_line_with_pos(ui.window_content_region_width() * 2.0 / 3.0 - 11.0);
                ui.text(label);
            }
        });
    }
}