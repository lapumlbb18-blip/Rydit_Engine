//! Rybot GUI — Inspector visual + Creador de proyectos con migui
//!
//! Paneles:
//! - **New Project**: nombre, template, directorio
//! - **Inspector**: propiedades del proyecto actual
//! - **Scene Tree**: nodos de la escena activa
//! - **Engine Stats**: FPS, frame time, nodos

use migui::{Migui, WidgetId, Rect, Color};
use crate::templates::ProjectTemplate;
use crate::RybotEngine;

// ============================================================================
// RYBOT GUI — Estado completo
// ============================================================================

/// Panel activo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    NewProject,
    Inspector,
    SceneTree,
    EngineStats,
    None,
}

/// Estado de la GUI de Rybot
pub struct RybotGui {
    // Paneles
    pub open: bool,
    pub active_panel: ActivePanel,

    // New Project
    pub project_name: String,
    pub selected_template: usize,
    pub project_dir: String,
    pub project_created: bool,
    pub status_message: String,
    pub status_ok: bool,

    // Window positions
    pub new_project_rect: Rect,
    pub inspector_rect: Rect,
    pub scene_tree_rect: Rect,
    pub stats_rect: Rect,

    // Menu bar
    pub menu_bar_open: bool,

    // Toggle flags
    pub show_new_project: bool,
    pub show_inspector: bool,
    pub show_scene_tree: bool,
    pub show_stats: bool,
    pub show_animation: bool,
    pub show_tilemap: bool,

    // Inspector sections (submenus)
    pub inspector_sections: std::collections::HashMap<String, bool>,

    // Window positions extra
    pub animation_rect: Rect,
    pub tilemap_rect: Rect,
}

impl Default for RybotGui {
    fn default() -> Self {
        Self::new()
    }
}

impl RybotGui {
    pub fn new() -> Self {
        let mut inspector_sections = std::collections::HashMap::new();
        inspector_sections.insert("MOTOR".to_string(), true);
        inspector_sections.insert("SUBSISTEMAS".to_string(), true);
        inspector_sections.insert("PROPIEDADES".to_string(), false);

        Self {
            open: false,
            active_panel: ActivePanel::None,
            project_name: String::from("mi_proyecto"),
            selected_template: 0,
            project_dir: String::new(),
            project_created: false,
            status_message: String::new(),
            status_ok: true,
            new_project_rect: Rect::new(20.0, 20.0, 400.0, 300.0),
            inspector_rect: Rect::new(820.0, 30.0, 350.0, 400.0),
            scene_tree_rect: Rect::new(820.0, 440.0, 350.0, 330.0),
            stats_rect: Rect::new(10.0, 600.0, 350.0, 150.0),
            animation_rect: Rect::new(10.0, 440.0, 800.0, 150.0),
            tilemap_rect: Rect::new(200.0, 100.0, 600.0, 400.0),
            menu_bar_open: true,
            show_new_project: false,
            show_inspector: true,
            show_scene_tree: true,
            show_stats: false,
            show_animation: true,
            show_tilemap: false,
            inspector_sections,
        }
    }

    /// Dibujar toda la GUI de Rybot
    pub fn draw(&mut self, gui: &mut Migui, stats: &crate::EngineStats, scene: &crate::SceneTree) {
        if !self.open {
            return;
        }

        // Toolkit / Toolbar (Acceso rápido táctil/mouse)
        self.draw_toolkit(gui);

        // Menu bar
        if self.menu_bar_open {
            self.draw_menu_bar(gui);
        }

        // Paneles
        if self.show_new_project {
            self.draw_new_project(gui);
        }
        if self.show_inspector {
            self.draw_inspector(gui, stats);
        }
        if self.show_scene_tree {
            self.draw_scene_tree(gui, scene);
        }
        if self.show_stats {
            self.draw_engine_stats(gui, stats);
        }
        if self.show_animation {
            self.draw_animation_panel(gui);
        }
        if self.show_tilemap {
            self.draw_tilemap_panel(gui);
        }
    }

    // ========================================================================
    // TOOLKIT / TOOLBAR (Quick Access)
    // ========================================================================

    fn draw_toolkit(&mut self, gui: &mut Migui) {
        let bar_w = 40.0;
        let bar_h = 240.0;
        let x = 0.0;
        let y = 100.0;

        // Fondo del Toolkit (barra lateral)
        gui.panel(
            WidgetId::new("toolkit_bg"),
            Rect::new(x, y, bar_w, bar_h),
            Color::new(50, 50, 60, 200),
        );

        let btn_size = 32.0;
        let mut curr_y = y + 5.0;
        let left = x + (bar_w - btn_size) / 2.0;

        // Botón 1: Animación
        if gui.button(WidgetId::new("tool_1"), Rect::new(left, curr_y, btn_size, btn_size), "🎬") {
            self.show_animation = !self.show_animation;
        }
        curr_y += btn_size + 5.0;

        // Botón 2: Tilemap
        if gui.button(WidgetId::new("tool_2"), Rect::new(left, curr_y, btn_size, btn_size), "🗺") {
            self.show_tilemap = !self.show_tilemap;
        }
        curr_y += btn_size + 5.0;

        // Botón 3: Inspector
        if gui.button(WidgetId::new("tool_3"), Rect::new(left, curr_y, btn_size, btn_size), "📋") {
            self.show_inspector = !self.show_inspector;
        }
        curr_y += btn_size + 5.0;

        // Botón 4: Scene Tree
        if gui.button(WidgetId::new("tool_4"), Rect::new(left, curr_y, btn_size, btn_size), "🌳") {
            self.show_scene_tree = !self.show_scene_tree;
        }
    }

    // ========================================================================
    // ANIMATION PANEL (Action Assets)
    // ========================================================================

    fn draw_animation_panel(&mut self, gui: &mut Migui) {
        let mut open = self.show_animation;
        if gui.window(
            WidgetId::new("win_animation"),
            "🎬 Línea de Tiempo / Action Assets",
            self.animation_rect,
            &mut open,
        ) {
            self.show_animation = open;
            let mut y = 60.0;
            let left = self.animation_rect.x + 15.0;
            
            gui.label(WidgetId::new("lbl_anim_placeholder"), 
                "Controles de Reproducción: [ ▶ ] [ ⏸ ] [ ⏹ ]  |  Hot Reload: [ 🔄 ]",
                Rect::new(left, y, self.animation_rect.w - 30.0, 20.0));
            y += 30.0;
            
            // Simulación de Timeline
            gui.panel(WidgetId::new("timeline_bg"), 
                Rect::new(left, y, self.animation_rect.w - 30.0, 40.0),
                Color::new(30, 30, 30, 255));
            
            // Cursor de tiempo
            gui.panel(WidgetId::new("timeline_cursor"),
                Rect::new(left + 50.0, y, 2.0, 40.0),
                Color::RED);
        }
    }

    // ========================================================================
    // TILEMAP PANEL
    // ========================================================================

    fn draw_tilemap_panel(&mut self, gui: &mut Migui) {
        let mut open = self.show_tilemap;
        if gui.window(
            WidgetId::new("win_tilemap"),
            "🗺 Tilemap Editor",
            self.tilemap_rect,
            &mut open,
        ) {
            self.show_tilemap = open;
            gui.label(WidgetId::new("lbl_tile_placeholder"), 
                "Paleta de Tiles: [ 🧱 ] [ 🌿 ] [ 💧 ] [ 🏠 ]",
                Rect::new(self.tilemap_rect.x + 15.0, 60.0, 200.0, 20.0));
        }
    }

    // ========================================================================
    // MENU BAR
    // ========================================================================

    fn draw_menu_bar(&mut self, gui: &mut Migui) {
        let bar_h = 25.0f32;
        gui.panel(
            WidgetId::new("menu_bg"),
            Rect::new(0.0, 0.0, 1200.0, bar_h),
            Color::new(40, 40, 40, 255),
        );

        let mut x = 5.0;
        let items = [
            ("Archivo", &["Nuevo Proyecto", "Abrir", "Guardar", "---", "Salir"] as &[&str]),
            ("Ver", &["Inspector", "Scene Tree", "Stats", "---", "Reset Layout"]),
            ("Ayuda", &["Documentación", "Acerca de Rybot"]),
        ];

        for (label, _subitems) in &items {
            let btn_w = 80.0;
            let rect = Rect::new(x, 2.0, btn_w, bar_h - 4.0);
            if gui.button(WidgetId::new(&format!("menu_{}", label)), rect, label) {
                match *label {
                    "Archivo" => self.show_new_project = !self.show_new_project,
                    "Ver" => {
                        self.show_inspector = !self.show_inspector;
                        self.show_scene_tree = !self.show_scene_tree;
                        self.show_stats = !self.show_stats;
                    }
                    "Ayuda" => {}
                    _ => {}
                }
            }
            x += btn_w + 5.0;
        }
    }

    // ========================================================================
    // NEW PROJECT PANEL
    // ========================================================================

    fn draw_new_project(&mut self, gui: &mut Migui) {
        let mut open = self.show_new_project;
        if gui.window(
            WidgetId::new("win_new_project"),
            "🛡️ Nuevo Proyecto",
            self.new_project_rect,
            &mut open,
        ) {
            self.show_new_project = open;

            let mut y = 60.0;
            let left = self.new_project_rect.x + 15.0;
            let field_w = self.new_project_rect.w - 30.0;

            // Nombre del proyecto
            gui.label(WidgetId::new("lbl_name"), "Nombre del proyecto:",
                Rect::new(left, y, 200.0, 20.0));
            y += 22.0;

            let name = gui.textbox(
                WidgetId::new("txt_name"),
                Rect::new(left, y, field_w, 28.0),
            ).to_string();
            if name != self.project_name {
                self.project_name = name;
                if self.project_dir.is_empty() {
                    self.project_dir = self.project_name.clone();
                }
            }
            y += 35.0;

            // Directorio
            gui.label(WidgetId::new("lbl_dir"), "Directorio:",
                Rect::new(left, y, 200.0, 20.0));
            y += 22.0;
            let dir_text = gui.textbox(
                WidgetId::new("txt_dir"),
                Rect::new(left, y, field_w, 28.0),
            ).to_string();
            self.project_dir = dir_text;
            y += 35.0;

            // Template selector
            gui.label(WidgetId::new("lbl_template"), "Template:",
                Rect::new(left, y, 200.0, 20.0));
            y += 22.0;

            let templates = ProjectTemplate::all();
            for (i, t) in templates.iter().enumerate() {
                let is_selected = i == self.selected_template;
                let label = format!("{} — {}", t.name(), t.description());
                let mut checked = is_selected;
                let radio_id = format!("radio_tpl_{}", i);
                // Usar checkbox como radio visual
                let _changed = gui.checkbox(
                    WidgetId::new(&radio_id),
                    &label,
                    &mut checked,
                    Rect::new(left, y, field_w, 20.0),
                );
                if checked && !is_selected {
                    self.selected_template = i;
                }
                y += 24.0;
            }
            y += 5.0;

            // Botón crear
            if gui.button(
                WidgetId::new("btn_create"),
                Rect::new(left, y, field_w, 32.0),
                "🚀 Crear Proyecto",
            ) {
                self.create_project();
            }
            y += 40.0;

            // Status
            if !self.status_message.is_empty() {
                let color = if self.status_ok { Color::GREEN } else { Color::RED };
                gui.label(WidgetId::new("lbl_status"), &self.status_message,
                    Rect::new(left, y, field_w, 20.0));
                // Override color for status — simplified (migui no tiene color override en label)
            }
        }
    }

    fn create_project(&mut self) {
        let dest = if self.project_dir.is_empty() {
            &self.project_name
        } else {
            &self.project_dir
        };

        let templates = ProjectTemplate::all();
        let template = if self.selected_template < templates.len() {
            templates[self.selected_template]
        } else {
            ProjectTemplate::Empty
        };

        match crate::templates::create_project(dest, &self.project_name, template) {
            Ok(()) => {
                self.status_message = format!("✅ Proyecto '{}' creado en {}/", self.project_name, dest);
                self.status_ok = true;
                self.project_created = true;
            }
            Err(e) => {
                self.status_message = format!("❌ {}", e);
                self.status_ok = false;
                self.project_created = false;
            }
        }
    }

    // ========================================================================
    // INSPECTOR PANEL
    // ========================================================================

    fn draw_inspector(&mut self, gui: &mut Migui, stats: &crate::EngineStats) {
        let mut open = self.show_inspector;
        if gui.window(
            WidgetId::new("win_inspector"),
            "📋 Inspector",
            self.inspector_rect,
            &mut open,
        ) {
            self.show_inspector = open;

            let mut y = 60.0;
            let left = self.inspector_rect.x + 15.0;
            let w = self.inspector_rect.w - 30.0;

            // --- SECCIÓN: MOTOR ---
            if self.collapsible_section(gui, "MOTOR", left, &mut y, w) {
                let state_label = match stats.state {
                    crate::EngineState::Running => "🟢 Running",
                    crate::EngineState::Paused => "🟡 Paused",
                    crate::EngineState::Loading => "🔵 Loading",
                    crate::EngineState::Initializing => "⚪ Initializing",
                    crate::EngineState::ShuttingDown => "🔴 ShuttingDown",
                };
                gui.label(WidgetId::new("ins_state"), &format!("Estado: {}", state_label),
                    Rect::new(left + 10.0, y, w - 10.0, 20.0));
                y += 20.0;
                gui.label(WidgetId::new("ins_frame"), &format!("Frame: {}", stats.frame),
                    Rect::new(left + 10.0, y, w - 10.0, 20.0));
                y += 20.0;

                // Botones de control del motor
                y += 5.0;
                if stats.state == crate::EngineState::Running {
                    let _ = gui.button(WidgetId::new("btn_pause"),
                        Rect::new(left + 10.0, y, (w - 10.0) / 2.0 - 5.0, 28.0), "⏸ Pausar");
                    let _ = gui.button(WidgetId::new("btn_shutdown"),
                        Rect::new(left + 10.0 + (w - 10.0) / 2.0 + 5.0, y, (w - 10.0) / 2.0 - 5.0, 28.0), "⏹ Detener");
                } else if stats.state == crate::EngineState::Paused {
                    let _ = gui.button(WidgetId::new("btn_resume"),
                        Rect::new(left + 10.0, y, w - 10.0, 28.0), "▶ Reanudar");
                }
                y += 35.0;
            }

            // --- SECCIÓN: SUBSISTEMAS ---
            if self.collapsible_section(gui, "SUBSISTEMAS", left, &mut y, w) {
                let subs = [
                    ("Input", stats.input_actions > 0),
                    ("Físicas", stats.physics_enabled),
                    ("Animación", stats.animation_enabled),
                    ("Red", stats.network_enabled),
                ];
                for (name, active) in &subs {
                    let status = if *active { "✅" } else { "❌" };
                    gui.label(WidgetId::new(&format!("sub_{}", name)),
                        &format!("{}  {}", status, name),
                        Rect::new(left + 10.0, y, w - 10.0, 20.0));
                    y += 20.0;
                }
                y += 5.0;
            }

            // --- SECCIÓN: PROPIEDADES ---
            if self.collapsible_section(gui, "PROPIEDADES", left, &mut y, w) {
                gui.label(WidgetId::new("ins_node_name"), "📦 Objeto: Cubo_Test",
                    Rect::new(left + 10.0, y, w - 10.0, 20.0));
                y += 22.0;

                // Transform
                gui.label(WidgetId::new("ins_pos"), &format!("Pos: {:.1}, {:.1}", stats.pan_x, stats.pan_y),
                    Rect::new(left + 20.0, y, w - 20.0, 20.0));
                y += 20.0;
                
                gui.label(WidgetId::new("ins_zoom"), &format!("Zoom: {:.2}x", stats.zoom),
                    Rect::new(left + 20.0, y, w - 20.0, 20.0));
                y += 20.0;
                
                gui.label(WidgetId::new("ins_scale"), "Escala: 1.0, 1.0, 1.0",
                    Rect::new(left + 20.0, y, w - 20.0, 20.0));
                y += 25.0;

                if gui.button(WidgetId::new("btn_reset_transform"),
                    Rect::new(left + 10.0, y, w - 10.0, 25.0), "🔄 Reset Transform") {
                    // Reset logic
                }
                y += 30.0;
            }
        }
    }

    /// Helper para dibujar una sección colapsable
    fn collapsible_section(&mut self, gui: &mut Migui, title: &str, x: f32, y: &mut f32, w: f32) -> bool {
        let is_open = *self.inspector_sections.get(title).unwrap_or(&false);
        let icon = if is_open { "▼" } else { "▶" };
        let label = format!("{} {}", icon, title);

        if gui.button(WidgetId::new(&format!("sec_btn_{}", title)),
            Rect::new(x, *y, w, 22.0), &label) {
            self.inspector_sections.insert(title.to_string(), !is_open);
        }
        *y += 24.0;

        is_open
    }

    // ========================================================================
    // SCENE TREE PANEL
    // ========================================================================

    fn draw_scene_tree(&mut self, gui: &mut Migui, scene: &crate::SceneTree) {
        let mut open = self.show_scene_tree;
        if gui.window(
            WidgetId::new("win_scene"),
            "🌳 Scene Tree",
            self.scene_tree_rect,
            &mut open,
        ) {
            self.show_scene_tree = open;

            let mut y = 60.0;
            let left = self.scene_tree_rect.x + 15.0;
            let w = self.scene_tree_rect.w - 30.0;

            let count = scene.node_count();
            gui.label(WidgetId::new("st_count"),
                &format!("Nodos: {}", count),
                Rect::new(left, y, w, 20.0));
            y += 25.0;

            // Lista de nodos
            self.draw_node_tree(gui, &scene.root, left, &mut y, 0);
        }
    }

    fn draw_node_tree(&self, gui: &mut Migui, node: &crate::SceneNode, x: f32, y: &mut f32, depth: usize) {
        let indent = depth as f32 * 16.0;
        let icon = match node.node_type {
            crate::NodeType::Root => "📁",
            crate::NodeType::Entity => "🎮",
            crate::NodeType::Camera => "📷",
            crate::NodeType::Light => "💡",
            crate::NodeType::UI => "🖼",
            crate::NodeType::Tilemap => "🗺",
            crate::NodeType::Particles => "✨",
            crate::NodeType::Audio => "🔊",
            crate::NodeType::Script => "📜",
        };

        let children_info = if node.children.is_empty() {
            String::new()
        } else {
            format!(" ({})", node.children.len())
        };
        let label = format!("{}{} {}{}",
            "  ".repeat(depth),
            icon,
            node.name,
            children_info,
        );

        gui.label(WidgetId::new(&format!("node_{}", node.name)),
            &label,
            Rect::new(x + indent, *y, self.scene_tree_rect.w - 30.0 - indent, 20.0));
        *y += 20.0;

        for child in &node.children {
            self.draw_node_tree(gui, child, x, y, depth + 1);
        }
    }

    // ========================================================================
    // ENGINE STATS PANEL
    // ========================================================================

    fn draw_engine_stats(&mut self, gui: &mut Migui, stats: &crate::EngineStats) {
        let mut open = self.show_stats;
        if gui.window(
            WidgetId::new("win_stats"),
            "📊 Engine Stats",
            self.stats_rect,
            &mut open,
        ) {
            self.show_stats = open;

            let mut y = 60.0;
            let left = self.stats_rect.x + 15.0;
            let w = self.stats_rect.w - 30.0;

            let lines = vec![
                format!("Frame: {} | Target FPS: {}", stats.frame, stats.target_fps),
                format!("Nodos escena: {}", stats.scene_nodes),
                format!("Acciones input: {}", stats.input_actions),
                format!("Físicas: {} | Animación: {} | Red: {}",
                    if stats.physics_enabled { "ON" } else { "OFF" },
                    if stats.animation_enabled { "ON" } else { "OFF" },
                    if stats.network_enabled { "ON" } else { "OFF" },
                ),
            ];

            for line in &lines {
                gui.label(WidgetId::new(&format!("stat_{}", line)), line,
                    Rect::new(left, y, w, 18.0));
                y += 18.0;
            }
        }
    }

    // ========================================================================
    // HELPERS
    // ========================================================================

    pub fn reset(&mut self) {
        self.project_name = String::from("mi_proyecto");
        self.selected_template = 0;
        self.project_dir = String::new();
        self.project_created = false;
        self.status_message = String::new();
        self.status_ok = true;
        self.show_new_project = false;
        self.show_inspector = true;
        self.show_scene_tree = true;
        self.show_stats = true;
    }

    pub fn selected_template_name(&self) -> &str {
        let templates = ProjectTemplate::all();
        if self.selected_template < templates.len() {
            templates[self.selected_template].name()
        } else {
            "empty"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_default_gui() {
        let gui = RybotGui::new();
        assert!(!gui.open);
        assert!(gui.show_inspector);
        assert!(gui.show_scene_tree);
        assert!(gui.show_stats);
    }

    #[test]
    fn test_create_project() {
        let mut gui = RybotGui::new();
        gui.open = true;
        gui.project_name = "test_gui_proj".into();
        gui.selected_template = 5;
        gui.create_project();
        assert!(gui.project_created);
        assert!(gui.status_ok);
        let _ = fs::remove_dir_all("test_gui_proj");
    }

    #[test]
    fn test_reset() {
        let mut gui = RybotGui::new();
        gui.project_name = "otro".into();
        gui.selected_template = 3;
        gui.reset();
        assert_eq!(gui.project_name, "mi_proyecto");
        assert_eq!(gui.selected_template, 0);
    }
}
