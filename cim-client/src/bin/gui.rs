use cim_client::{MCPClient, Ontology, Result, Term, Relationship};
use iced::{
    alignment, executor, widget::{Column, Container, Row, Text, Button, TextInput, Scrollable, PickList,
    scrollable, text_input, Container as IcedContainer, horizontal_rule},
    Application, Command, Element, Length, Settings, Theme, Alignment, Color, Subscription, time,
};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::runtime::Runtime;
use serde_json::Value;

// Create a runtime for async operations
thread_local! {
    static RUNTIME: Runtime = Runtime::new().expect("Failed to create Tokio runtime");
}

#[derive(Debug, Clone)]
enum Message {
    // General
    None,
    Tick(Instant),
    ErrorOccurred(String),
    
    // Navigation
    NavigateToOntologies,
    NavigateToOntology(String),
    NavigateToCreateOntology,
    
    // Ontology list
    FetchOntologies,
    OntologiesFetched(Vec<Ontology>),
    
    // Ontology detail
    FetchOntology(String),
    OntologyFetched(Ontology),
    DeleteOntology(String),
    OntologyDeleted(String),
    
    // Create ontology
    NameChanged(String),
    DescriptionChanged(String),
    DomainChanged(String),
    DomainSelected(String),
    SubmitOntology,
    OntologyCreated(Ontology),
    Cancel,
    
    // Create term
    TermNameChanged(String),
    TermDescriptionChanged(String),
    SubmitTerm,
    TermCreated(Term),
    
    // Create relationship
    SourceTermSelected(String),
    TargetTermSelected(String),
    RelationshipTypeChanged(String),
    SubmitRelationship,
    RelationshipCreated(Relationship),
}

#[derive(Debug, Clone)]
enum Page {
    OntologiesList,
    OntologyDetail(String),
    CreateOntology,
}

struct OntologyApp {
    // App state
    page: Page,
    client: MCPClient,
    error: Option<String>,
    last_update: Instant,
    
    // Ontologies list
    ontologies: Vec<Ontology>,
    is_loading_ontologies: bool,
    
    // Ontology detail
    current_ontology: Option<Ontology>,
    is_loading_ontology: bool,
    
    // Create ontology
    new_ontology_name: String,
    new_ontology_description: String,
    new_ontology_domain: String,
    available_domains: Vec<String>,
    
    // Create term
    new_term_name: String,
    new_term_description: String,
    
    // Create relationship
    selected_source_term: Option<String>,
    selected_target_term: Option<String>,
    new_relationship_type: String,
    relationship_types: Vec<String>,
}

impl Default for OntologyApp {
    fn default() -> Self {
        Self {
            // App state
            page: Page::OntologiesList,
            client: RUNTIME.with(|rt| rt.block_on(async {
                MCPClient::new("nats://localhost:4222").await.expect("Failed to connect to NATS")
            })),
            error: None,
            last_update: Instant::now(),
            
            // Ontologies list
            ontologies: Vec::new(),
            is_loading_ontologies: false,
            
            // Ontology detail
            current_ontology: None,
            is_loading_ontology: false,
            
            // Create ontology
            new_ontology_name: String::new(),
            new_ontology_description: String::new(),
            new_ontology_domain: String::new(),
            available_domains: vec![
                "AI".to_string(),
                "Business".to_string(),
                "Technology".to_string(),
                "Science".to_string(),
                "Healthcare".to_string(),
                "Other".to_string(),
            ],
            
            // Create term
            new_term_name: String::new(),
            new_term_description: String::new(),
            
            // Create relationship
            selected_source_term: None,
            selected_target_term: None,
            new_relationship_type: String::new(),
            relationship_types: vec![
                "is_a".to_string(),
                "part_of".to_string(),
                "related_to".to_string(),
                "has_property".to_string(),
                "depends_on".to_string(),
            ],
        }
    }
}

impl Application for OntologyApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::perform(async {}, |_| Message::FetchOntologies),
        )
    }

    fn title(&self) -> String {
        String::from("CIM Ontology Client")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::None => Command::none(),
            
            Message::Tick(_) => {
                // Refresh data periodically
                if self.last_update.elapsed() > Duration::from_secs(30) {
                    self.last_update = Instant::now();
                    return match self.page {
                        Page::OntologiesList => {
                            Command::perform(async {}, |_| Message::FetchOntologies)
                        },
                        Page::OntologyDetail(ref id) => {
                            let id_clone = id.clone();
                            Command::perform(async move { id_clone }, Message::FetchOntology)
                        },
                        _ => Command::none(),
                    };
                }
                Command::none()
            },
            
            Message::ErrorOccurred(error) => {
                self.error = Some(error);
                Command::none()
            },
            
            // Navigation
            Message::NavigateToOntologies => {
                self.page = Page::OntologiesList;
                self.current_ontology = None;
                Command::perform(async {}, |_| Message::FetchOntologies)
            },
            
            Message::NavigateToOntology(id) => {
                self.page = Page::OntologyDetail(id.clone());
                Command::perform(async move { id }, Message::FetchOntology)
            },
            
            Message::NavigateToCreateOntology => {
                self.page = Page::CreateOntology;
                self.new_ontology_name = String::new();
                self.new_ontology_description = String::new();
                self.new_ontology_domain = String::new();
                Command::none()
            },
            
            // Ontology list
            Message::FetchOntologies => {
                self.is_loading_ontologies = true;
                let client = self.client.clone();
                Command::perform(
                    async move {
                        client.list_ontologies().await
                    },
                    |result| match result {
                        Ok(ontologies) => Message::OntologiesFetched(ontologies),
                        Err(e) => Message::ErrorOccurred(e.to_string()),
                    },
                )
            },
            
            Message::OntologiesFetched(ontologies) => {
                self.is_loading_ontologies = false;
                self.ontologies = ontologies;
                Command::none()
            },
            
            // Ontology detail
            Message::FetchOntology(id) => {
                self.is_loading_ontology = true;
                let client = self.client.clone();
                Command::perform(
                    async move {
                        client.get_ontology(&id).await
                    },
                    |result| match result {
                        Ok(ontology) => Message::OntologyFetched(ontology),
                        Err(e) => Message::ErrorOccurred(e.to_string()),
                    },
                )
            },
            
            Message::OntologyFetched(ontology) => {
                self.is_loading_ontology = false;
                self.current_ontology = Some(ontology);
                Command::none()
            },
            
            Message::DeleteOntology(id) => {
                let client = self.client.clone();
                Command::perform(
                    async move {
                        client.delete_ontology(&id).await.map(|_| id)
                    },
                    |result| match result {
                        Ok(id) => Message::OntologyDeleted(id),
                        Err(e) => Message::ErrorOccurred(e.to_string()),
                    },
                )
            },
            
            Message::OntologyDeleted(_) => {
                Command::perform(async {}, |_| Message::NavigateToOntologies)
            },
            
            // Create ontology
            Message::NameChanged(name) => {
                self.new_ontology_name = name;
                Command::none()
            },
            
            Message::DescriptionChanged(description) => {
                self.new_ontology_description = description;
                Command::none()
            },
            
            Message::DomainChanged(domain) => {
                self.new_ontology_domain = domain;
                Command::none()
            },
            
            Message::DomainSelected(domain) => {
                self.new_ontology_domain = domain;
                Command::none()
            },
            
            Message::SubmitOntology => {
                let name = self.new_ontology_name.clone();
                let description = self.new_ontology_description.clone();
                let domain = self.new_ontology_domain.clone();
                
                if name.is_empty() || description.is_empty() || domain.is_empty() {
                    return Command::perform(
                        async { "Please fill in all fields".to_string() },
                        Message::ErrorOccurred,
                    );
                }
                
                let client = self.client.clone();
                Command::perform(
                    async move {
                        client.create_ontology(&name, &description, &domain).await
                    },
                    |result| match result {
                        Ok(ontology) => Message::OntologyCreated(ontology),
                        Err(e) => Message::ErrorOccurred(e.to_string()),
                    },
                )
            },
            
            Message::OntologyCreated(ontology) => {
                let id = ontology.id.clone();
                Command::perform(async move { id }, Message::NavigateToOntology)
            },
            
            Message::Cancel => {
                Command::perform(async {}, |_| Message::NavigateToOntologies)
            },
            
            // Create term
            Message::TermNameChanged(name) => {
                self.new_term_name = name;
                Command::none()
            },
            
            Message::TermDescriptionChanged(description) => {
                self.new_term_description = description;
                Command::none()
            },
            
            Message::SubmitTerm => {
                if let Some(ontology) = &self.current_ontology {
                    let name = self.new_term_name.clone();
                    let description = self.new_term_description.clone();
                    let ontology_id = ontology.id.clone();
                    
                    if name.is_empty() || description.is_empty() {
                        return Command::perform(
                            async { "Please fill in all fields".to_string() },
                            Message::ErrorOccurred,
                        );
                    }
                    
                    let client = self.client.clone();
                    Command::perform(
                        async move {
                            client.create_term(&ontology_id, &name, &description, None).await
                        },
                        |result| match result {
                            Ok(term) => Message::TermCreated(term),
                            Err(e) => Message::ErrorOccurred(e.to_string()),
                        },
                    )
                } else {
                    Command::none()
                }
            },
            
            Message::TermCreated(_) => {
                if let Page::OntologyDetail(id) = &self.page {
                    let id = id.clone();
                    self.new_term_name.clear();
                    self.new_term_description.clear();
                    Command::perform(async move { id }, Message::FetchOntology)
                } else {
                    Command::none()
                }
            },
            
            // Create relationship
            Message::SourceTermSelected(term_id) => {
                self.selected_source_term = Some(term_id);
                Command::none()
            },
            
            Message::TargetTermSelected(term_id) => {
                self.selected_target_term = Some(term_id);
                Command::none()
            },
            
            Message::RelationshipTypeChanged(rel_type) => {
                self.new_relationship_type = rel_type;
                Command::none()
            },
            
            Message::SubmitRelationship => {
                if let (Some(ontology), Some(source), Some(target)) = (
                    &self.current_ontology,
                    &self.selected_source_term,
                    &self.selected_target_term,
                ) {
                    let ontology_id = ontology.id.clone();
                    let source_id = source.clone();
                    let target_id = target.clone();
                    let rel_type = self.new_relationship_type.clone();
                    
                    if rel_type.is_empty() {
                        return Command::perform(
                            async { "Please specify a relationship type".to_string() },
                            Message::ErrorOccurred,
                        );
                    }
                    
                    let client = self.client.clone();
                    Command::perform(
                        async move {
                            client.create_relationship(&ontology_id, &rel_type, &source_id, &target_id, None).await
                        },
                        |result| match result {
                            Ok(relationship) => Message::RelationshipCreated(relationship),
                            Err(e) => Message::ErrorOccurred(e.to_string()),
                        },
                    )
                } else {
                    Command::perform(
                        async { "Please select both terms".to_string() },
                        Message::ErrorOccurred,
                    )
                }
            },
            
            Message::RelationshipCreated(_) => {
                if let Page::OntologyDetail(id) = &self.page {
                    let id = id.clone();
                    self.selected_source_term = None;
                    self.selected_target_term = None;
                    self.new_relationship_type = String::new();
                    Command::perform(async move { id }, Message::FetchOntology)
                } else {
                    Command::none()
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match &self.page {
            Page::OntologiesList => self.view_ontologies_list(),
            Page::OntologyDetail(_) => self.view_ontology_detail(),
            Page::CreateOntology => self.view_create_form(),
        };
        
        let main_content = Column::new()
            .push(self.view_header())
            .push(content)
            .padding(20)
            .spacing(20)
            .width(Length::Fill);
            
        if let Some(error) = &self.error {
            Container::new(
                Column::new()
                    .push(main_content)
                    .push(
                        Container::new(
                            Text::new(format!("Error: {}", error))
                                .size(16)
                                .color([0.8, 0.0, 0.0])
                        )
                        .padding(10)
                        .style(|_| iced::widget::container::Appearance {
                            background: Some(iced::Background::Color([1.0, 0.9, 0.9].into())),
                            border_radius: 5.0,
                            border_width: 1.0,
                            border_color: [0.8, 0.0, 0.0].into(),
                            ..Default::default()
                        })
                        .width(Length::Fill)
                    )
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            Container::new(main_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(Message::Tick)
    }
}

impl OntologyApp {
    fn view_header(&self) -> Element<Message> {
        let title = Text::new("CIM Ontology Client")
            .size(28)
            .width(Length::Fill);
            
        let home_button = Button::new(Text::new("Home"))
            .on_press(Message::NavigateToOntologies);
            
        let create_button = Button::new(Text::new("New Ontology"))
            .on_press(Message::NavigateToCreateOntology);
            
        let header_row = Row::new()
            .push(title)
            .push(home_button)
            .push(create_button)
            .align_items(Alignment::Center)
            .spacing(10);
            
        Container::new(header_row)
            .width(Length::Fill)
            .style(|_| iced::widget::container::Appearance {
                background: Some(iced::Background::Color([0.9, 0.9, 0.95].into())),
                border_radius: 5.0,
                border_width: 0.0,
                ..Default::default()
            })
            .padding(10)
            .into()
    }
    
    fn view_ontologies_list(&self) -> Element<Message> {
        let title = Text::new("Ontologies")
            .size(24)
            .width(Length::Fill);
            
        let mut content = Column::new()
            .push(title)
            .push(horizontal_rule(1))
            .spacing(10);
            
        if self.is_loading_ontologies {
            content = content.push(Text::new("Loading...").size(16));
        } else if self.ontologies.is_empty() {
            content = content.push(
                Container::new(
                    Column::new()
                        .push(Text::new("No ontologies found").size(16))
                        .push(
                            Button::new(Text::new("Create New Ontology"))
                                .on_press(Message::NavigateToCreateOntology)
                        )
                        .spacing(10)
                        .align_items(Alignment::Center)
                )
                .width(Length::Fill)
                .center_x()
                .padding(20)
            );
        } else {
            let mut list = Column::new().spacing(10);
            
            for ontology in &self.ontologies {
                let row = Container::new(
                    Row::new()
                        .push(
                            Column::new()
                                .push(Text::new(&ontology.name).size(18))
                                .push(Text::new(&ontology.description).size(14))
                                .push(Text::new(format!("Domain: {}", &ontology.domain)).size(12))
                                .width(Length::Fill)
                                .spacing(5)
                        )
                        .push(
                            Button::new(Text::new("View"))
                                .on_press(Message::NavigateToOntology(ontology.id.clone()))
                        )
                        .align_items(Alignment::Center)
                        .spacing(10)
                )
                .style(|_| iced::widget::container::Appearance {
                    background: Some(iced::Background::Color([0.95, 0.95, 1.0].into())),
                    border_radius: 5.0,
                    border_width: 1.0,
                    border_color: [0.8, 0.8, 0.9].into(),
                    ..Default::default()
                })
                .padding(10)
                .width(Length::Fill);
                
                list = list.push(row);
            }
            
            content = content.push(
                Scrollable::new(list)
                    .height(Length::Fill)
                    .width(Length::Fill)
            );
        }
        
        content.into()
    }
    
    fn view_ontology_detail(&self) -> Element<Message> {
        if self.is_loading_ontology {
            return Column::new()
                .push(Text::new("Loading ontology...").size(16))
                .into();
        }
        
        if let Some(ontology) = &self.current_ontology {
            let header = Column::new()
                .push(
                    Row::new()
                        .push(Text::new(&ontology.name).size(24).width(Length::Fill))
                        .push(
                            Button::new(Text::new("Delete"))
                                .on_press(Message::DeleteOntology(ontology.id.clone()))
                                .style(iced::theme::Button::Destructive)
                        )
                        .align_items(Alignment::Center)
                )
                .push(Text::new(format!("Domain: {}", &ontology.domain)).size(16))
                .push(Text::new(&ontology.description).size(14))
                .push(
                    Text::new(format!(
                        "Created: {} | Updated: {}", 
                        &ontology.created_at, &ontology.updated_at
                    )).size(12)
                )
                .spacing(5);
                
            let tabs = Row::new()
                .push(Text::new("Terms").size(18))
                .push(Text::new("Relationships").size(18))
                .spacing(20)
                .padding(10);
                
            // Terms section
            let terms_list = self.view_terms_list(ontology);
            
            // Relationships section
            let relationships_list = self.view_relationships_list(ontology);
            
            // Create term form
            let create_term_form = Column::new()
                .push(Text::new("Add New Term").size(18))
                .push(
                    TextInput::new("Term name", &self.new_term_name)
                        .on_input(Message::TermNameChanged)
                        .padding(8)
                )
                .push(
                    TextInput::new("Term description", &self.new_term_description)
                        .on_input(Message::TermDescriptionChanged)
                        .padding(8)
                )
                .push(
                    Button::new(Text::new("Add Term"))
                        .on_press(Message::SubmitTerm)
                        .width(Length::Fill)
                )
                .spacing(10)
                .max_width(400);
                
            // Create relationship form
            let term_options: Vec<(String, String)> = ontology.terms.iter()
                .map(|t| (t.id.clone(), t.name.clone()))
                .collect();
                
            let source_term_pick = if !term_options.is_empty() {
                let mut column = Column::new()
                    .push(Text::new("Source Term").size(14));
                    
                // Simplified PickList implementation
                let mut row = Row::new().spacing(5);
                for (id, name) in &term_options {
                    let is_selected = self.selected_source_term.as_ref().map_or(false, |s| s == id);
                    let button = Button::new(Text::new(name))
                        .on_press(Message::SourceTermSelected(id.clone()))
                        .style(if is_selected {
                            iced::theme::Button::Primary
                        } else {
                            iced::theme::Button::Secondary
                        });
                    row = row.push(button);
                }
                column.push(row)
            } else {
                Column::new().push(Text::new("No terms available").size(14))
            };
            
            let target_term_pick = if !term_options.is_empty() {
                let mut column = Column::new()
                    .push(Text::new("Target Term").size(14));
                    
                // Simplified PickList implementation
                let mut row = Row::new().spacing(5);
                for (id, name) in &term_options {
                    let is_selected = self.selected_target_term.as_ref().map_or(false, |s| s == id);
                    let button = Button::new(Text::new(name))
                        .on_press(Message::TargetTermSelected(id.clone()))
                        .style(if is_selected {
                            iced::theme::Button::Primary
                        } else {
                            iced::theme::Button::Secondary
                        });
                    row = row.push(button);
                }
                column.push(row)
            } else {
                Column::new().push(Text::new("No terms available").size(14))
            };
            
            let relationship_type_input = Column::new()
                .push(Text::new("Relationship Type").size(14))
                .push(
                    TextInput::new("e.g. is_a, part_of", &self.new_relationship_type)
                        .on_input(Message::RelationshipTypeChanged)
                        .padding(8)
                );
                
            let relationship_buttons = Row::new()
                .spacing(5);
                
            let create_relationship_form = Column::new()
                .push(Text::new("Add New Relationship").size(18))
                .push(source_term_pick)
                .push(target_term_pick)
                .push(relationship_type_input)
                .push(
                    Button::new(Text::new("Add Relationship"))
                        .on_press(Message::SubmitRelationship)
                        .width(Length::Fill)
                )
                .spacing(10)
                .max_width(400);
                
            Column::new()
                .push(header)
                .push(horizontal_rule(1))
                .push(tabs)
                .push(
                    Row::new()
                        .push(
                            Column::new()
                                .push(terms_list)
                                .push(relationships_list)
                                .spacing(20)
                                .width(Length::FillPortion(3))
                        )
                        .push(
                            Column::new()
                                .push(create_term_form)
                                .push(horizontal_rule(1))
                                .push(create_relationship_form)
                                .spacing(20)
                                .width(Length::FillPortion(2))
                        )
                        .spacing(20)
                )
                .spacing(10)
                .into()
        } else {
            Column::new()
                .push(Text::new("Ontology not found").size(16))
                .push(
                    Button::new(Text::new("Back to Ontologies"))
                        .on_press(Message::NavigateToOntologies)
                )
                .spacing(10)
                .into()
        }
    }
    
    fn view_terms_list(&self, ontology: &Ontology) -> Element<Message> {
        let title = Text::new("Terms").size(20);
        
        let content = if ontology.terms.is_empty() {
            Container::new(
                Text::new("No terms defined").size(14)
            )
            .padding(10)
            .width(Length::Fill)
        } else {
            let mut list = Column::new().spacing(5);
            
            for term in &ontology.terms {
                let row = Container::new(
                    Column::new()
                        .push(Text::new(&term.name).size(16))
                        .push(Text::new(&term.description).size(14))
                        .spacing(5)
                )
                .style(|_| iced::widget::container::Appearance {
                    background: Some(iced::Background::Color([0.95, 0.98, 0.95].into())),
                    border_radius: 5.0,
                    border_width: 1.0,
                    border_color: [0.8, 0.9, 0.8].into(),
                    ..Default::default()
                })
                .padding(10)
                .width(Length::Fill);
                
                list = list.push(row);
            }
            
            Container::new(
                Scrollable::new(list)
                    .height(Length::Units(300))
            )
            .width(Length::Fill)
        };
        
        Column::new()
            .push(title)
            .push(content)
            .spacing(10)
            .into()
    }
    
    fn view_relationships_list(&self, ontology: &Ontology) -> Element<Message> {
        let title = Text::new("Relationships").size(20);
        
        let content = if ontology.relationships.is_empty() {
            Container::new(
                Text::new("No relationships defined").size(14)
            )
            .padding(10)
            .width(Length::Fill)
        } else {
            let mut list = Column::new().spacing(5);
            
            for relationship in &ontology.relationships {
                // Find source and target term names
                let source_name = ontology.terms.iter()
                    .find(|t| t.id == relationship.source_term_id)
                    .map(|t| t.name.clone())
                    .unwrap_or_else(|| relationship.source_term_id.clone());
                    
                let target_name = ontology.terms.iter()
                    .find(|t| t.id == relationship.target_term_id)
                    .map(|t| t.name.clone())
                    .unwrap_or_else(|| relationship.target_term_id.clone());
                
                let row = Container::new(
                    Column::new()
                        .push(
                            Text::new(format!("{} → {} → {}", 
                                source_name, 
                                relationship.relationship_type,
                                target_name
                            )).size(16)
                        )
                        .spacing(5)
                )
                .style(|_| iced::widget::container::Appearance {
                    background: Some(iced::Background::Color([0.95, 0.95, 1.0].into())),
                    border_radius: 5.0,
                    border_width: 1.0,
                    border_color: [0.8, 0.8, 0.9].into(),
                    ..Default::default()
                })
                .padding(10)
                .width(Length::Fill);
                
                list = list.push(row);
            }
            
            Container::new(
                Scrollable::new(list)
                    .height(Length::Units(200))
            )
            .width(Length::Fill)
        };
        
        Column::new()
            .push(title)
            .push(content)
            .spacing(10)
            .into()
    }
    
    fn view_create_form(&self) -> Element<Message> {
        let title = Text::new("Create New Ontology")
            .size(24)
            .width(Length::Fill);
            
        let name_input = TextInput::new("Ontology name", &self.new_ontology_name)
            .on_input(Message::NameChanged)
            .padding(8);
            
        let description_input = TextInput::new("Ontology description", &self.new_ontology_description)
            .on_input(Message::DescriptionChanged)
            .padding(8);
            
        let domain_input = TextInput::new("Domain", &self.new_ontology_domain)
            .on_input(Message::DomainChanged)
            .padding(8);
            
        // Domain Buttons
        let domain_buttons = Row::new()
            .spacing(5);
            
        let domain_buttons = self.available_domains.iter().fold(domain_buttons, |row, domain| {
            let is_selected = &self.new_ontology_domain == domain;
            row.push(
                Button::new(Text::new(domain))
                    .on_press(Message::DomainSelected(domain.clone()))
                    .style(if is_selected {
                        iced::theme::Button::Primary
                    } else {
                        iced::theme::Button::Secondary
                    })
            )
        });
            
        let button_row = Row::new()
            .push(
                Button::new(Text::new("Cancel"))
                    .on_press(Message::Cancel)
                    .width(Length::Fill)
            )
            .push(
                Button::new(Text::new("Create"))
                    .on_press(Message::SubmitOntology)
                    .width(Length::Fill)
            )
            .spacing(10);
            
        Column::new()
            .push(title)
            .push(horizontal_rule(1))
            .push(name_input)
            .push(description_input)
            .push(domain_input)
            .push(Text::new("Or select a domain:").size(14))
            .push(domain_buttons)
            .push(button_row)
            .max_width(500)
            .spacing(10)
            .into()
    }
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Run the Iced application
    OntologyApp::run(Settings {
        window: iced::window::Settings {
            size: (1024, 768),
            position: iced::window::Position::Centered,
            min_size: Some((800, 600)),
            ..Default::default()
        },
        default_font: None,
        antialiasing: true,
        ..Default::default()
    })?;

    Ok(())
} 