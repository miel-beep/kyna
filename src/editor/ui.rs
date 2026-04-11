use ratatui::widgets::Widget;

use crate::editor::{KynaScene, SceneId};

pub struct KynaUi {
    scenes: Vec<Box<dyn KynaScene>>,
    current_scene: SceneId,
}

impl KynaUi {
    pub fn new() -> Self {
        Self {
            scenes: vec![],
            current_scene: SceneId(usize::MAX), //none
        }
    }

    #[inline]
    ///Returns whether this ui has some ui active
    pub fn has_active_scene(&self) -> bool {
        self.current_scene.0 != usize::MAX
    }
    pub fn set_active_scene(&mut self, id: SceneId) {
        self.current_scene = id;
    }

    pub fn get_active_scene(&mut self) -> Option<&mut Box<dyn KynaScene>> {
        self.scenes.get_mut(self.current_scene.0)
    }

    pub fn add_scene<S: KynaScene + 'static>(&mut self, scene: S, active: bool) -> SceneId {
        self.scenes.push(Box::new(scene));
        let id = SceneId(self.scenes.len() - 1);
        if active {
            self.current_scene = id;
        };
        id
    }
}

impl Widget for &KynaUi {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if self.has_active_scene() {
            let scene = &*self.scenes[self.current_scene.0];
            scene.render(area, buf);
        }
    }
}
