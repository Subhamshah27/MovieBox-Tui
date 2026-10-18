use eframe::egui;
use moviebox_tui::providers::{CatalogItem, ProviderKind};
use moviebox_tui::service::MovieBoxService;

#[derive(Default)]
struct MovieBoxDesktop {
    query: String,
    active_page: Page,
    status: String,
    provider: ProviderKind,
    results: Vec<CatalogItem>,
    searching: bool,
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

impl MovieBoxDesktop {
    fn search(&mut self) {
        let query = self.query.trim().to_owned();
        if query.is_empty() {
            self.status = "Enter a title to search.".to_owned();
            return;
        }
        self.searching = true;
        self.status = format!("Searching {}…", self.provider.label());
        let provider = self.provider;
        let result = std::thread::Builder::new()
            .name("moviebox-search".to_owned())
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .map_err(|e| e.to_string())?;
                let service = MovieBoxService::new();
                runtime.block_on(service.search_typed(provider, &query, 1))
                    .map_err(|e| e.to_string())
            });
        match result {
            Ok(handle) => match handle.join() {
                Ok(Ok(items)) => {
                    self.results = items;
                    self.status = format!("Found {} results from {}.", self.results.len(), self.provider.label());
                }
                Ok(Err(err)) => self.status = format!("Search failed: {err}"),
                Err(_) => self.status = "Search worker stopped unexpectedly.".to_owned(),
            },
            Err(err) => self.status = format!("Could not start search: {err}"),
        }
        self.searching = false;
    }
}

impl eframe::App for MovieBoxDesktop {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("MOVIEBOX").strong().color(egui::Color32::from_rgb(111, 170, 255)));
                ui.label(egui::RichText::new("DESKTOP").small().weak());
                ui.add_space(16.0);
                ui.add_sized([ui.available_width().min(360.0), 34.0], egui::TextEdit::singleline(&mut self.query).hint_text("Search movies, series, anime…"));
                egui::ComboBox::from_id_salt("provider")
                    .selected_text(self.provider.label())
                    .show_ui(ui, |ui| {
                        for provider in ProviderKind::ENABLED {
                            ui.selectable_value(&mut self.provider, provider, provider.label());
                        }
                    });
                if ui.add_enabled(!self.searching, egui::Button::new("Search")).clicked() {
                    self.search();
                }
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
                ui.label(egui::RichText::new("Search uses existing Rust providers").small().weak());
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
            ui.add_space(18.0);

            if !self.status.is_empty() {
                ui.label(egui::RichText::new(&self.status).color(egui::Color32::from_rgb(240, 196, 90)));
                ui.add_space(8.0);
            }

            if self.results.is_empty() {
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(28, 43, 68))
                    .corner_radius(egui::CornerRadius::same(14))
                    .inner_margin(egui::Margin::same(22))
                    .show(ui, |ui| {
                        ui.set_min_height(100.0);
                        ui.label(egui::RichText::new(if self.active_page == Page::Home { "Find something to watch" } else { self.active_page.label() }).size(22.0).strong());
                        ui.add_space(8.0);
                        ui.label("Search a title above to retrieve live results from the existing MovieBox provider.");
                    });
            } else {
                ui.heading("Search results");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for item in &self.results {
                        egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(&item.title).strong().size(16.0));
                                    let kind = match item.media_type {
                                        moviebox_tui::providers::MediaType::Movie => "Movie",
                                        moviebox_tui::providers::MediaType::Series => "Series",
                                    };
                                    ui.label(format!("{}  •  {}  •  {}", kind, item.year.as_deref().unwrap_or("Year unknown"), item.id.provider.label()));
                                    ui.label(egui::RichText::new(format!("Provider ID: {}", item.id.value)).small().weak());
                                });
                            });
                        });
                        ui.add_space(4.0);
                    }
                });
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.separator();
                ui.label(egui::RichText::new("Early desktop build • Playback and installer still require validation.").small().weak());
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
