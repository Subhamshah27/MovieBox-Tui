use eframe::egui;

#[derive(Default)]
struct MovieBoxDesktop {
    query: String,
    active_page: Page,
    status: String,
}

#[derive(Default, PartialEq, Clone, Copy)]
enum Page {
    #[default]
    Home,
    Movies,
    Series,
    Anime,
    LiveTv,
    Downloads,
    Library,
    Settings,
}

impl Page {
    fn label(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Movies => "Movies",
            Self::Series => "TV Series",
            Self::Anime => "Anime",
            Self::LiveTv => "Live TV",
            Self::Downloads => "Downloads",
            Self::Library => "My Library",
            Self::Settings => "Settings",
        }
    }
}

impl eframe::App for MovieBoxDesktop {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("MOVIEBOX").strong().color(egui::Color32::from_rgb(111, 170, 255)));
                ui.label(egui::RichText::new("DESKTOP").small().weak());
                ui.add_space(24.0);
                ui.add_sized([ui.available_width().min(420.0), 34.0], egui::TextEdit::singleline(&mut self.query).hint_text("Search movies, series, anime…"));
                if ui.button("Search").clicked() {
                    self.status = if self.query.trim().is_empty() {
                        "Enter a title to search.".to_owned()
                    } else {
                        format!("Search UI ready for “{}”. Provider integration is the next step.", self.query.trim())
                    };
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("Local-first • No account required").small().weak());
                });
            });
        });

        egui::SidePanel::left("sidebar").resizable(false).default_width(185.0).show(ctx, |ui| {
            ui.add_space(12.0);
            ui.label(egui::RichText::new("DISCOVER").small().strong().weak());
            for page in [Page::Home, Page::Movies, Page::Series, Page::Anime, Page::LiveTv] {
                if ui.selectable_label(self.active_page == page, format!("   {}", page.label())).clicked() {
                    self.active_page = page;
                }
            }
            ui.add_space(18.0);
            ui.label(egui::RichText::new("YOUR SPACE").small().strong().weak());
            for page in [Page::Library, Page::Downloads, Page::Settings] {
                if ui.selectable_label(self.active_page == page, format!("   {}", page.label())).clicked() {
                    self.active_page = page;
                }
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.separator();
                ui.label(egui::RichText::new("MovieBox desktop preview").small().weak());
                ui.label(egui::RichText::new("Built on the existing Rust core").small().weak());
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading(self.active_page.label());
            ui.label(egui::RichText::new(match self.active_page {
                Page::Home => "Your entertainment, in one place.",
                Page::Movies => "Browse and search movies.",
                Page::Series => "Find your next series.",
                Page::Anime => "Explore anime.",
                Page::LiveTv => "Live channels and TV guides.",
                Page::Downloads => "Manage downloads and progress.",
                Page::Library => "Continue watching, favorites and history.",
                Page::Settings => "Playback, providers and application preferences.",
            }).color(egui::Color32::from_gray(165)));
            ui.add_space(22.0);

            if self.active_page == Page::Home {
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(28, 43, 68))
                    .corner_radius(egui::CornerRadius::same(14))
                    .inner_margin(egui::Margin::same(22))
                    .show(ui, |ui| {
                        ui.set_min_height(145.0);
                        ui.label(egui::RichText::new("A better way to find what to watch").size(24.0).strong());
                        ui.add_space(8.0);
                        ui.label("A modern desktop interface backed by the MovieBox Rust project.");
                        ui.add_space(12.0);
                        if ui.button("Browse movies").clicked() {
                            self.active_page = Page::Movies;
                        }
                    });
                ui.add_space(24.0);
                ui.heading("Explore");
                ui.add_space(8.0);
                ui.columns(4, |columns| {
                    for (i, (name, detail, page)) in [
                        ("Movies", "Films and new discoveries", Page::Movies),
                        ("TV Series", "Pick up your next series", Page::Series),
                        ("Anime", "Explore animated stories", Page::Anime),
                        ("Live TV", "Channels and EPG", Page::LiveTv),
                    ].iter().enumerate() {
                        egui::Frame::group(columns[i].style())
                            .corner_radius(egui::CornerRadius::same(10))
                            .show(&mut columns[i], |ui| {
                                ui.set_min_height(100.0);
                                ui.label(egui::RichText::new(*name).strong());
                                ui.add_space(6.0);
                                ui.label(egui::RichText::new(*detail).small().weak());
                                ui.add_space(10.0);
                                if ui.button("Open").clicked() {
                                    self.active_page = *page;
                                }
                            });
                    }
                });
            } else {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.set_min_height(110.0);
                    ui.label(egui::RichText::new(self.active_page.label()).size(18.0).strong());
                    ui.add_space(8.0);
                    ui.label("This screen is scaffolded. The next implementation step is to connect it to the existing provider, library, download and playback services.");
                });
            }

            ui.add_space(18.0);
            if !self.status.is_empty() {
                ui.label(egui::RichText::new(&self.status).color(egui::Color32::from_rgb(240, 196, 90)));
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.separator();
                ui.label(egui::RichText::new("Preview build — streaming and provider actions are not connected yet.").small().weak());
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("MovieBox Desktop")
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([860.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "MovieBox Desktop",
        options,
        Box::new(|_cc| Ok(Box::new(MovieBoxDesktop::default()))),
    )
}
