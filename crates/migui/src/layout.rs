//! # Layout Flexbox para MiGUI
//!
//! Layout automático tipo CSS Flexbox / Bevy UI / Godot Control.
//! Sin posicionamiento manual — los widgets se alinean automáticamente.
//!
//! ## Filosofía
//! - **Immediate mode**: cada frame se recalcula desde cero
//! - **Simple**: row/col, padding, spacing, align, grow
//! - **Sin nesting compleja**: containers planos (nested = v2)
//!
//! ## Ejemplo
//!
//! ```rust,ignore
//! // Container horizontal con padding y spacing
//! layout.begin_container(FlexDir::Row, Rect::new(10, 10, 400, 60));
//! layout.set_padding(10.0);
//! layout.set_spacing(8.0);
//! layout.set_align_items(LayoutAlign::Center);
//!
//! // Items con grow
//! layout.add_item(FlexSize::Grow(1.0)); // botón ocupa espacio proporcional
//! layout.add_item(FlexSize::Fixed(80.0)); // botón tamaño fijo
//! layout.add_item(FlexSize::Grow(2.0)); // botón ocupa el doble
//!
//! // Obtener rects calculados
//! let rects = layout.compute();
//! // rects[0], rects[1], rects[2] son las posiciones de cada item
//! ```

use crate::{Migui, Rect, Color, WidgetId};

// ============================================================================
// ENUMS DE LAYOUT
// ============================================================================

/// Dirección del layout
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexDir {
    /// Horizontal (izquierda → derecha)
    #[default]
    Row,
    /// Vertical (arriba → abajo)
    Column,
}

/// Alineación en el eje principal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexAlignMain {
    /// Inicio del container
    #[default]
    Start,
    /// Centro del container
    Center,
    /// Final del container
    End,
    /// Espaciado uniforme entre items
    SpaceBetween,
    /// Espaciado uniforme alrededor de items
    SpaceAround,
}

/// Alineación en el eje cruzado
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexAlignCross {
    /// Inicio del eje cruzado
    Start,
    /// Centro del eje cruzado
    #[default]
    Center,
    /// Final del eje cruzado
    End,
    /// Estirar para llenar
    Stretch,
}

/// Tamaño de un item
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexSize {
    /// Tamaño fijo en píxeles
    Fixed(f32),
    /// Proporción de espacio restante (como flex-grow)
    Grow(f32),
    /// Tamaño automático (basado en contenido — por ahora = Fixed)
    Auto,
}

impl Default for FlexSize {
    fn default() -> Self {
        FlexSize::Auto
    }
}

// ============================================================================
// ITEM DE LAYOUT
// ============================================================================

/// Item individual dentro de un container
#[derive(Debug, Clone, Default)]
pub struct FlexItem {
    pub size: FlexSize,
    /// Padding interno del item (override del container)
    pub padding_override: Option<f32>,
    pub id: Option<String>,
}

impl FlexItem {
    pub fn new(size: FlexSize) -> Self {
        Self { size, padding_override: None, id: None }
    }

    pub fn grow(factor: f32) -> Self {
        Self::new(FlexSize::Grow(factor))
    }

    pub fn fixed(pixels: f32) -> Self {
        Self::new(FlexSize::Fixed(pixels))
    }

    pub fn auto() -> Self {
        Self::new(FlexSize::Auto)
    }
}

// ============================================================================
// CONTAINER DE LAYOUT
// ============================================================================

/// Container Flexbox
#[derive(Debug, Clone)]
pub struct FlexContainer {
    pub direction: FlexDir,
    pub rect: Rect,
    pub items: Vec<FlexItem>,
    /// Padding interno del container
    pub padding: f32,
    /// Espaciado entre items
    pub spacing: f32,
    /// Alineación eje principal
    pub align_main: FlexAlignMain,
    /// Alineación eje cruzado
    pub align_cross: FlexAlignCross,
    /// Color de fondo (debug)
    pub bg_color: Option<Color>,
}

impl Default for FlexContainer {
    fn default() -> Self {
        Self::new(FlexDir::Row, Rect::new(0.0, 0.0, 400.0, 60.0))
    }
}

impl FlexContainer {
    pub fn new(direction: FlexDir, rect: Rect) -> Self {
        Self {
            direction,
            rect,
            items: Vec::new(),
            padding: 0.0,
            spacing: 0.0,
            align_main: FlexAlignMain::default(),
            align_cross: FlexAlignCross::default(),
            bg_color: None,
        }
    }

    /// Agregar item al container
    pub fn add_item(&mut self, item: FlexItem) {
        self.items.push(item);
    }

    /// Calcular posiciones de todos los items
    /// Retorna Vec de Rect con las posiciones calculadas
    pub fn compute(&self) -> Vec<Rect> {
        if self.items.is_empty() {
            return Vec::new();
        }

        let mut rects = Vec::with_capacity(self.items.len());
        let inner_w = self.rect.w - self.padding * 2.0;
        let inner_h = self.rect.h - self.padding * 2.0;

        // Paso 1: Calcular tamaño base de items fijos y total grow
        let mut fixed_total = 0.0;
        let mut total_grow = 0.0;

        for item in &self.items {
            match item.size {
                FlexSize::Fixed(s) => fixed_total += s,
                FlexSize::Auto => fixed_total += 50.0, // default auto size
                FlexSize::Grow(g) => total_grow += g,
            }
        }

        // Espacio total de spacing
        let spacing_total = self.spacing * (self.items.len().saturating_sub(1)) as f32;

        // Espacio disponible para items grow
        let main_available = if self.direction == FlexDir::Row {
            inner_w - fixed_total - spacing_total
        } else {
            inner_h - fixed_total - spacing_total
        }
        .max(0.0);

        // Paso 2: Calcular tamaño de cada item en el eje principal
        let mut main_sizes = Vec::with_capacity(self.items.len());
        for item in &self.items {
            let size = match item.size {
                FlexSize::Fixed(s) => s,
                FlexSize::Auto => 50.0,
                FlexSize::Grow(g) => {
                    if total_grow > 0.0 {
                        main_available * (g / total_grow)
                    } else {
                        0.0
                    }
                }
            };
            main_sizes.push(size.max(0.0));
        }

        // Paso 3: Calcular offset inicial según align_main
        let total_main: f32 = main_sizes.iter().sum::<f32>() + spacing_total;
        let extra_space = (if self.direction == FlexDir::Row { inner_w } else { inner_h }) - total_main;

        let main_start = match self.align_main {
            FlexAlignMain::Start => 0.0,
            FlexAlignMain::Center => extra_space / 2.0,
            FlexAlignMain::End => extra_space,
            FlexAlignMain::SpaceBetween => 0.0, // first item at start
            FlexAlignMain::SpaceAround => {
                if self.items.len() > 0 {
                    extra_space / (self.items.len() as f32 * 2.0)
                } else {
                    0.0
                }
            }
        };

        // Paso 4: Posicionar items
        let mut main_offset = self.padding + main_start.max(0.0);

        if self.direction == FlexDir::Row {
            for (i, item) in self.items.iter().enumerate() {
                let w = main_sizes[i];
                let h = inner_h;

                // Alineación cruzada (vertical para Row)
                let y = match self.align_cross {
                    FlexAlignCross::Start => self.padding,
                    FlexAlignCross::Center => self.padding + (inner_h - h) / 2.0,
                    FlexAlignCross::End => self.padding + inner_h - h,
                    FlexAlignCross::Stretch => self.padding,
                };

                rects.push(Rect::new(
                    self.rect.x + main_offset,
                    self.rect.y + y,
                    w, h,
                ));

                main_offset += w + self.spacing;
                // Extra spacing for SpaceBetween / SpaceAround
                if i < self.items.len() - 1 {
                    match self.align_main {
                        FlexAlignMain::SpaceBetween => {
                            if self.items.len() > 1 {
                                main_offset += (extra_space / (self.items.len() - 1) as f32).max(0.0);
                            }
                        }
                        FlexAlignMain::SpaceAround => {
                            main_offset += (extra_space / (self.items.len() as f32 * 2.0)).max(0.0);
                        }
                        _ => {}
                    }
                }
            }
        } else {
            // Column
            for (i, item) in self.items.iter().enumerate() {
                let h = main_sizes[i];
                let w = inner_w;

                // Alineación cruzada (horizontal para Column)
                let x = match self.align_cross {
                    FlexAlignCross::Start => self.padding,
                    FlexAlignCross::Center => self.padding + (inner_w - w) / 2.0,
                    FlexAlignCross::End => self.padding + inner_w - w,
                    FlexAlignCross::Stretch => self.padding,
                };

                rects.push(Rect::new(
                    self.rect.x + x,
                    self.rect.y + main_offset,
                    w, h,
                ));

                main_offset += h + self.spacing;
                // Extra spacing for SpaceBetween / SpaceAround
                if i < self.items.len() - 1 {
                    match self.align_main {
                        FlexAlignMain::SpaceBetween => {
                            if self.items.len() > 1 {
                                main_offset += (extra_space / (self.items.len() - 1) as f32).max(0.0);
                            }
                        }
                        FlexAlignMain::SpaceAround => {
                            main_offset += (extra_space / (self.items.len() as f32 * 2.0)).max(0.0);
                        }
                        _ => {}
                    }
                }
            }
        }

        rects
    }
}

// ============================================================================
// LAYOUT MANAGER
// ============================================================================

/// Gestor de layouts — permite registrar containers y usarlos con Migui
pub struct FlexManager {
    containers: Vec<FlexContainer>,
}

impl Default for FlexManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FlexManager {
    pub fn new() -> Self {
        Self { containers: Vec::new() }
    }

    /// Crear un nuevo container y retornar su índice
    pub fn begin_container(&mut self, direction: FlexDir, rect: Rect) -> usize {
        self.containers.push(FlexContainer::new(direction, rect));
        self.containers.len() - 1
    }

    /// Obtener referencia mutable al container activo
    pub fn container_mut(&mut self, idx: usize) -> &mut FlexContainer {
        &mut self.containers[idx]
    }

    /// Calcular todos los containers y retornar Vec de Vec de Rects
    pub fn compute_all(&self) -> Vec<Vec<Rect>> {
        self.containers.iter().map(|c| c.compute()).collect()
    }

    /// Limpiar todos los containers (llamar al inicio de cada frame)
    pub fn clear(&mut self) {
        self.containers.clear();
    }

    /// Debug: dibujar containers con colores
    pub fn draw_debug(&self, gui: &mut Migui) {
        for (ci, container) in self.containers.iter().enumerate() {
            // Fondo del container
            if let Some(bg) = container.bg_color {
                gui.panel(
                    WidgetId::new(&format!("layout_bg_{}", ci)),
                    container.rect,
                    bg,
                );
            }

            // Items calculados
            let rects = container.compute();
            let colors = [
                Color::new(80, 80, 120, 100),
                Color::new(80, 120, 80, 100),
                Color::new(120, 80, 80, 100),
                Color::new(120, 120, 80, 100),
                Color::new(80, 120, 120, 100),
                Color::new(120, 80, 120, 100),
            ];
            for (ri, rect) in rects.iter().enumerate() {
                let c = colors[ri % colors.len()];
                gui.panel(
                    WidgetId::new(&format!("layout_item_{}_{}", ci, ri)),
                    *rect,
                    c,
                );
                // Borde
                gui.panel(
                    WidgetId::new(&format!("layout_border_{}_{}", ci, ri)),
                    Rect::new(rect.x, rect.y, rect.w, 2.0),
                    Color::new(200, 200, 200, 150),
                );
            }
        }
    }
}

// ============================================================================
// HELPERS
// ============================================================================

/// Helper: crear container horizontal
pub fn row(x: f32, y: f32, w: f32, h: f32) -> FlexContainer {
    FlexContainer::new(FlexDir::Row, Rect::new(x, y, w, h))
}

/// Helper: crear container vertical
pub fn col(x: f32, y: f32, w: f32, h: f32) -> FlexContainer {
    FlexContainer::new(FlexDir::Column, Rect::new(x, y, w, h))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_layout_fixed() {
        let mut container = FlexContainer::new(FlexDir::Row, Rect::new(0.0, 0.0, 400.0, 60.0));
        container.padding = 10.0;
        container.spacing = 5.0;
        container.add_item(FlexItem::fixed(100.0));
        container.add_item(FlexItem::fixed(100.0));
        container.add_item(FlexItem::fixed(100.0));

        let rects = container.compute();
        assert_eq!(rects.len(), 3);
        // 400 - 20 padding = 380 disponible
        // 3 items de 100 = 300, 2 spacing de 5 = 10 → total 310
        // offset = 10 (padding)
        assert!((rects[0].x - 10.0).abs() < 0.1);
        assert!((rects[1].x - 115.0).abs() < 0.1); // 10 + 100 + 5
        assert!((rects[2].x - 220.0).abs() < 0.1); // 115 + 100 + 5
        assert!((rects[0].w - 100.0).abs() < 0.1);
        assert!((rects[0].h - 40.0).abs() < 0.1); // 60 - 20 padding
    }

    #[test]
    fn test_row_layout_grow() {
        let mut container = FlexContainer::new(FlexDir::Row, Rect::new(0.0, 0.0, 400.0, 60.0));
        container.padding = 0.0;
        container.spacing = 0.0;
        container.add_item(FlexItem::grow(1.0));
        container.add_item(FlexItem::grow(1.0));

        let rects = container.compute();
        assert_eq!(rects.len(), 2);
        assert!((rects[0].w - 200.0).abs() < 0.1);
        assert!((rects[1].w - 200.0).abs() < 0.1);
        assert!((rects[1].x - 200.0).abs() < 0.1);
    }

    #[test]
    fn test_row_layout_grow_unequal() {
        let mut container = FlexContainer::new(FlexDir::Row, Rect::new(0.0, 0.0, 300.0, 60.0));
        container.add_item(FlexItem::grow(1.0));
        container.add_item(FlexItem::grow(2.0));

        let rects = container.compute();
        assert!((rects[0].w - 100.0).abs() < 0.1); // 1/3 de 300
        assert!((rects[1].w - 200.0).abs() < 0.1); // 2/3 de 300
    }

    #[test]
    fn test_column_layout() {
        let mut container = FlexContainer::new(FlexDir::Column, Rect::new(0.0, 0.0, 200.0, 300.0));
        container.padding = 10.0;
        container.spacing = 10.0;
        container.add_item(FlexItem::fixed(80.0));
        container.add_item(FlexItem::fixed(80.0));

        let rects = container.compute();
        assert!((rects[0].y - 10.0).abs() < 0.1);
        assert!((rects[1].y - 100.0).abs() < 0.1); // 10 + 80 + 10
        assert!((rects[0].h - 80.0).abs() < 0.1);
        assert!((rects[0].w - 180.0).abs() < 0.1); // 200 - 20 padding
    }

    #[test]
    fn test_align_center() {
        let mut container = FlexContainer::new(FlexDir::Row, Rect::new(0.0, 0.0, 400.0, 60.0));
        container.align_main = FlexAlignMain::Center;
        container.add_item(FlexItem::fixed(100.0));
        container.add_item(FlexItem::fixed(100.0));

        let rects = container.compute();
        // Total = 200, container = 400, offset = 100
        assert!((rects[0].x - 100.0).abs() < 0.1);
        assert!((rects[1].x - 200.0).abs() < 0.1);
    }

    #[test]
    fn test_align_end() {
        let mut container = FlexContainer::new(FlexDir::Row, Rect::new(0.0, 0.0, 400.0, 60.0));
        container.align_main = FlexAlignMain::End;
        container.add_item(FlexItem::fixed(100.0));
        container.add_item(FlexItem::fixed(100.0));

        let rects = container.compute();
        // Total = 200, container = 400, offset = 200
        assert!((rects[0].x - 200.0).abs() < 0.1);
        assert!((rects[1].x - 300.0).abs() < 0.1);
    }

    #[test]
    fn test_align_cross_center() {
        // For Column, items take full width (inner_w). Center has no effect since w == inner_w.
        let mut container = FlexContainer::new(FlexDir::Column, Rect::new(0.0, 0.0, 200.0, 300.0));
        container.add_item(FlexItem::auto());
        let rects = container.compute();
        // With 1 Auto item (50px height), full width
        assert_eq!(rects.len(), 1);
        assert!((rects[0].x - 0.0).abs() < 0.1); // full width, no centering
        assert!((rects[0].w - 200.0).abs() < 0.1);
    }

    #[test]
    fn test_align_cross_center_row() {
        // For Row, items take full height. Test with fixed height item.
        let mut container = FlexContainer::new(FlexDir::Row, Rect::new(0.0, 0.0, 400.0, 100.0));
        container.align_cross = FlexAlignCross::Center;
        container.add_item(FlexItem::fixed(80.0)); // width = 80

        let rects = container.compute();
        // item h = full inner_h = 100, so center y = 0
        assert!((rects[0].y - 0.0).abs() < 0.1);
        assert!((rects[0].h - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_space_between() {
        let mut container = FlexContainer::new(FlexDir::Row, Rect::new(0.0, 0.0, 400.0, 60.0));
        container.align_main = FlexAlignMain::SpaceBetween;
        container.spacing = 0.0;
        container.add_item(FlexItem::fixed(50.0));
        container.add_item(FlexItem::fixed(50.0));
        container.add_item(FlexItem::fixed(50.0));

        let rects = container.compute();
        // 3 items de 50 = 150, espacio disponible = 250
        // 2 gaps = 125 cada uno
        // First at 0, second at 50+125=175, third at 175+50+125=350
        assert!((rects[0].x - 0.0).abs() < 0.1);
        assert!((rects[1].x - 175.0).abs() < 0.1);
        assert!((rects[2].x - 350.0).abs() < 0.1);
    }

    #[test]
    fn test_empty_container() {
        let container = FlexContainer::new(FlexDir::Row, Rect::new(0.0, 0.0, 100.0, 100.0));
        let rects = container.compute();
        assert!(rects.is_empty());
    }

    #[test]
    fn test_layout_manager() {
        let mut lm = FlexManager::new();
        let idx = lm.begin_container(FlexDir::Row, Rect::new(0.0, 0.0, 400.0, 60.0));
        lm.container_mut(idx).add_item(FlexItem::grow(1.0));
        lm.container_mut(idx).add_item(FlexItem::grow(1.0));

        let all = lm.compute_all();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].len(), 2);

        lm.clear();
        assert!(lm.compute_all().is_empty());
    }

    #[test]
    fn test_mixed_fixed_and_grow() {
        let mut container = FlexContainer::new(FlexDir::Row, Rect::new(0.0, 0.0, 500.0, 60.0));
        container.add_item(FlexItem::fixed(100.0));
        container.add_item(FlexItem::grow(1.0));
        container.add_item(FlexItem::fixed(100.0));

        let rects = container.compute();
        assert!((rects[0].w - 100.0).abs() < 0.1);
        assert!((rects[0].x - 0.0).abs() < 0.1);
        assert!((rects[1].w - 300.0).abs() < 0.1); // 500 - 100 - 100
        assert!((rects[1].x - 100.0).abs() < 0.1);
        assert!((rects[2].w - 100.0).abs() < 0.1);
        assert!((rects[2].x - 400.0).abs() < 0.1);
    }

    #[test]
    fn test_layout_helpers() {
        let r = row(10.0, 20.0, 300.0, 50.0);
        assert_eq!(r.direction, FlexDir::Row);
        assert!((r.rect.x - 10.0).abs() < 0.1);

        let c = col(10.0, 20.0, 300.0, 50.0);
        assert_eq!(c.direction, FlexDir::Column);
    }

    #[test]
    fn test_layout_item_helpers() {
        let g = FlexItem::grow(2.0);
        assert!(matches!(g.size, FlexSize::Grow(2.0)));

        let f = FlexItem::fixed(50.0);
        assert!(matches!(f.size, FlexSize::Fixed(50.0)));

        let a = FlexItem::auto();
        assert!(matches!(a.size, FlexSize::Auto));
    }
}
