use crate::config::DotfileEntry;
use crate::scanner::DotfileScanner;
use color_eyre::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    Home,
    Browse,
    Preview,
    Installing,
}

pub struct App {
    pub view: View,
    pub dotfiles: Vec<DotfileEntry>,
    pub selected_index: usize,
    pub should_quit: bool,
    pub install_output: Option<String>,
    pub install_success: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let scanner = DotfileScanner::new();
        let dotfiles = scanner.scan()?;

        Ok(Self {
            view: View::Home,
            dotfiles,
            selected_index: 0,
            should_quit: false,
            install_output: None,
            install_success: false,
        })
    }

    pub fn next_item(&mut self) {
        if !self.dotfiles.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.dotfiles.len();
        }
    }

    pub fn previous_item(&mut self) {
        if !self.dotfiles.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.dotfiles.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn selected_dotfile(&self) -> Option<&DotfileEntry> {
        self.dotfiles.get(self.selected_index)
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn go_to_view(&mut self, view: View) {
        self.view = view;
    }

    pub fn go_back(&mut self) {
        self.view = match self.view {
            View::Home => View::Home,
            View::Browse => View::Home,
            View::Preview => View::Browse,
            View::Installing => View::Preview,
        };
    }

    pub fn install_selected(&mut self) -> Result<()> {
        if let Some(entry) = self.selected_dotfile() {
            use crate::installer::install_dotfile;

            let result = install_dotfile(entry)?;
            self.install_output = Some(result.output);
            self.install_success = result.success;
            self.view = View::Installing;
        }
        Ok(())
    }
}
