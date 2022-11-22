use plotly::{
    common::{Anchor, ColorScalePalette, Visible},
    layout::{
        update_menu::{ButtonBuilder, UpdateMenu, UpdateMenuDirection, UpdateMenuType},
        BarMode,
    },
    Bar, HeatMap, Layout, Plot,
};

/// Display a heat map, with buttons to allow for toggling of different
/// colorscales.
pub fn heat_map_with_modifiable_colorscale() {
    type HeatMapType = HeatMap<f64, f64, Vec<f64>>;
    let gauss = |v: i32| (-v as f64 * v as f64 / 200.0).exp();
    let z = (-30..30)
        .map(|x| (-30..30).map(|y| gauss(x) * gauss(y)).collect())
        .collect();
    let trace = HeatMapType::new_z(z).color_scale(ColorScalePalette::Viridis.into());
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let buttons = IntoIterator::into_iter([
        ("Viridis", ColorScalePalette::Viridis),
        ("Portland", ColorScalePalette::Portland),
        ("Blackbody", ColorScalePalette::Blackbody),
    ])
    .map(|(label, palette)| {
        ButtonBuilder::new()
            .label(label)
            .push_restyle(HeatMapType::modify_all_color_scale(palette.into()))
            .build()
    })
    .collect();

    plot.set_layout(Layout::new().update_menus(vec![UpdateMenu::new()
            .ty(UpdateMenuType::Buttons)
            .y(0.8)
            .buttons(buttons)]));

    plot.show();
}
