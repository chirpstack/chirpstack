use std::str::FromStr;

use tonic::{Request, Response, Status};
use uuid::Uuid;

use chirpstack_api::api;
use chirpstack_api::api::application_service_server::ApplicationService;

use super::auth::validator;
use super::error::ToStatus;
use super::helpers;
use crate::certificate;
use crate::storage::application;

pub struct Application {
    validator: validator::RequestValidator,
}

impl Application {
    pub fn new(validator: validator::RequestValidator) -> Self {
        Application { validator }
    }
}

#[tonic::async_trait]
impl ApplicationService for Application {
    async fn create(
        &self,
        request: Request<api::CreateApplicationRequest>,
    ) -> Result<Response<api::CreateApplicationResponse>, Status> {
        let req_app = match &request.get_ref().application {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("application is missing"));
            }
        };
        let tenant_id = Uuid::from_str(&req_app.tenant_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationsAccess::new(validator::Flag::Create, tenant_id),
            )
            .await?;

        let a = application::Application {
            tenant_id,
            name: req_app.name.clone(),
            description: req_app.description.clone(),
            ..Default::default()
        };

        let a = application::create(a).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::CreateApplicationResponse {
            id: a.id.to_string(),
        });
        resp.metadata_mut()
            .insert("x-log-application_id", a.id.to_string().parse().unwrap());

        Ok(resp)
    }

    async fn get(
        &self,
        request: Request<api::GetApplicationRequest>,
    ) -> Result<Response<api::GetApplicationResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let a = application::get(&app_id).await.map_err(|e| e.status())?;
        let measurement_keys = application::get_measurement_keys(&app_id)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::GetApplicationResponse {
            application: Some(api::Application {
                id: a.id.to_string(),
                tenant_id: a.tenant_id.to_string(),
                name: a.name,
                description: a.description,
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&a.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&a.updated_at)),
            measurement_keys,
        });
        resp.metadata_mut()
            .insert("x-log-application_id", req.id.parse().unwrap());

        Ok(resp)
    }

    async fn update(
        &self,
        request: Request<api::UpdateApplicationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_app = match &request.get_ref().application {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("application is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_app.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::update(application::Application {
            id: app_id,
            name: req_app.name.to_string(),
            description: req_app.description.to_string(),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req_app.id.parse().unwrap());

        Ok(resp)
    }

    async fn delete(
        &self,
        request: Request<api::DeleteApplicationRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Delete, app_id),
            )
            .await?;

        application::delete(&app_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req.id.parse().unwrap());

        Ok(resp)
    }

    async fn list(
        &self,
        request: Request<api::ListApplicationsRequest>,
    ) -> Result<Response<api::ListApplicationsResponse>, Status> {
        let req = request.get_ref();
        let tenant_id = Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationsAccess::new(validator::Flag::List, tenant_id),
            )
            .await?;

        let filters = application::Filters {
            tenant_id: Some(tenant_id),
            search: if req.search.is_empty() {
                None
            } else {
                Some(req.search.to_string())
            },
        };

        let count = application::get_count(&filters)
            .await
            .map_err(|e| e.status())?;
        let results = application::list(req.limit as i64, req.offset as i64, &filters)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListApplicationsResponse {
            total_count: count as u32,
            result: results
                .iter()
                .map(|a| api::ApplicationListItem {
                    id: a.id.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&a.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&a.updated_at)),
                    name: a.name.clone(),
                    description: a.description.clone(),
                })
                .collect(),
        });
        resp.metadata_mut()
            .insert("x-log-tenant_id", req.tenant_id.parse().unwrap());

        Ok(resp)
    }

    async fn list_integrations(
        &self,
        request: Request<api::ListIntegrationsRequest>,
    ) -> Result<Response<api::ListIntegrationsResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let result = application::get_integrations_for_application(&app_id)
            .await
            .map_err(|e| e.status())?;

        let mut items: Vec<api::IntegrationListItem> = result
            .iter()
            .map(|i| api::IntegrationListItem {
                kind: match i.kind {
                    application::IntegrationKind::Http => api::IntegrationKind::Http,
                    application::IntegrationKind::InfluxDb => api::IntegrationKind::InfluxDb,
                    application::IntegrationKind::ThingsBoard => api::IntegrationKind::ThingsBoard,
                    application::IntegrationKind::MyDevices => api::IntegrationKind::MyDevices,
                    application::IntegrationKind::LoraCloud => api::IntegrationKind::LoraCloud,
                    application::IntegrationKind::GcpPubSub => api::IntegrationKind::GcpPubSub,
                    application::IntegrationKind::AwsSns => api::IntegrationKind::AwsSns,
                    application::IntegrationKind::AzureServiceBus => {
                        api::IntegrationKind::AzureServiceBus
                    }
                    application::IntegrationKind::PilotThings => api::IntegrationKind::PilotThings,
                    application::IntegrationKind::Ifttt => api::IntegrationKind::Ifttt,
                }
                .into(),
            })
            .collect();
        items.push(api::IntegrationListItem {
            kind: api::IntegrationKind::MqttGlobal.into(),
        });

        let mut resp = Response::new(api::ListIntegrationsResponse {
            total_count: (result.len() + 1) as u32,
            result: items,
        });
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn create_http_integration(
        &self,
        request: Request<api::CreateHttpIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let i = application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::Http,
            configuration: application::IntegrationConfiguration::Http(
                application::HttpConfiguration {
                    headers: req_int.headers.clone(),
                    json: match req_int.encoding() {
                        api::Encoding::Protobuf => false,
                        api::Encoding::Json => true,
                    },
                    event_endpoint_url: req_int.event_endpoint_url.clone(),
                },
            ),
            ..Default::default()
        };

        let _ = application::create_integration(i)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get_http_integration(
        &self,
        request: Request<api::GetHttpIntegrationRequest>,
    ) -> Result<Response<api::GetHttpIntegrationResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let i = application::get_integration(&app_id, application::IntegrationKind::Http)
            .await
            .map_err(|e| e.status())?;

        if let application::IntegrationConfiguration::Http(conf) = &i.configuration {
            let mut resp = Response::new(api::GetHttpIntegrationResponse {
                integration: Some(api::HttpIntegration {
                    application_id: app_id.to_string(),
                    headers: conf.headers.clone(),
                    encoding: match conf.json {
                        true => api::Encoding::Json,
                        false => api::Encoding::Protobuf,
                    }
                    .into(),
                    event_endpoint_url: conf.event_endpoint_url.clone(),
                }),
            });
            resp.metadata_mut()
                .insert("x-log-application_id", req.application_id.parse().unwrap());

            Ok(resp)
        } else {
            Err(Status::internal("Integration has no Http configuration"))
        }
    }

    async fn update_http_integration(
        &self,
        request: Request<api::UpdateHttpIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::update_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::Http,
            configuration: application::IntegrationConfiguration::Http(
                application::HttpConfiguration {
                    headers: req_int.headers.clone(),
                    json: match req_int.encoding() {
                        api::Encoding::Protobuf => false,
                        api::Encoding::Json => true,
                    },
                    event_endpoint_url: req_int.event_endpoint_url.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn delete_http_integration(
        &self,
        request: Request<api::DeleteHttpIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        application::delete_integration(&app_id, application::IntegrationKind::Http)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn create_influx_db_integration(
        &self,
        request: Request<api::CreateInfluxDbIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let i = application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::InfluxDb,
            configuration: application::IntegrationConfiguration::InfluxDb(
                application::InfluxDbConfiguration {
                    endpoint: req_int.endpoint.clone(),
                    db: req_int.db.clone(),
                    username: req_int.username.clone(),
                    password: req_int.password.clone(),
                    retention_policy_name: req_int.retention_policy_name.clone(),
                    precision: req_int.precision,
                    version: req_int.version,
                    token: req_int.token.clone(),
                    organization: req_int.organization.clone(),
                    bucket: req_int.bucket.clone(),
                },
            ),
            ..Default::default()
        };

        let _ = application::create_integration(i)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get_influx_db_integration(
        &self,
        request: Request<api::GetInfluxDbIntegrationRequest>,
    ) -> Result<Response<api::GetInfluxDbIntegrationResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let i = application::get_integration(&app_id, application::IntegrationKind::InfluxDb)
            .await
            .map_err(|e| e.status())?;

        if let application::IntegrationConfiguration::InfluxDb(conf) = &i.configuration {
            let mut resp = Response::new(api::GetInfluxDbIntegrationResponse {
                integration: Some(api::InfluxDbIntegration {
                    application_id: app_id.to_string(),
                    endpoint: conf.endpoint.clone(),
                    db: conf.db.clone(),
                    username: conf.username.clone(),
                    password: conf.password.clone(),
                    retention_policy_name: conf.retention_policy_name.clone(),
                    precision: conf.precision,
                    version: conf.version,
                    token: conf.token.clone(),
                    organization: conf.organization.clone(),
                    bucket: conf.bucket.clone(),
                }),
            });
            resp.metadata_mut()
                .insert("x-log-application_id", req.application_id.parse().unwrap());

            Ok(resp)
        } else {
            Err(Status::internal(
                "Integration has no InfluxDb configuration",
            ))
        }
    }

    async fn update_influx_db_integration(
        &self,
        request: Request<api::UpdateInfluxDbIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::update_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::InfluxDb,
            configuration: application::IntegrationConfiguration::InfluxDb(
                application::InfluxDbConfiguration {
                    endpoint: req_int.endpoint.clone(),
                    db: req_int.db.clone(),
                    username: req_int.username.clone(),
                    password: req_int.password.clone(),
                    retention_policy_name: req_int.retention_policy_name.clone(),
                    precision: req_int.precision,
                    version: req_int.version,
                    token: req_int.token.clone(),
                    organization: req_int.organization.clone(),
                    bucket: req_int.bucket.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn delete_influx_db_integration(
        &self,
        request: Request<api::DeleteInfluxDbIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        application::delete_integration(&app_id, application::IntegrationKind::InfluxDb)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn create_things_board_integration(
        &self,
        request: Request<api::CreateThingsBoardIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let i = application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::ThingsBoard,
            configuration: application::IntegrationConfiguration::ThingsBoard(
                application::ThingsBoardConfiguration {
                    server: req_int.server.clone(),
                },
            ),
            ..Default::default()
        };

        let _ = application::create_integration(i)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get_things_board_integration(
        &self,
        request: Request<api::GetThingsBoardIntegrationRequest>,
    ) -> Result<Response<api::GetThingsBoardIntegrationResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let i = application::get_integration(&app_id, application::IntegrationKind::ThingsBoard)
            .await
            .map_err(|e| e.status())?;

        if let application::IntegrationConfiguration::ThingsBoard(conf) = &i.configuration {
            let mut resp = Response::new(api::GetThingsBoardIntegrationResponse {
                integration: Some(api::ThingsBoardIntegration {
                    application_id: app_id.to_string(),
                    server: conf.server.clone(),
                }),
            });
            resp.metadata_mut()
                .insert("x-log-application_id", req.application_id.parse().unwrap());

            Ok(resp)
        } else {
            Err(Status::internal(
                "Integration has no ThingsBoard configuration",
            ))
        }
    }

    async fn update_things_board_integration(
        &self,
        request: Request<api::UpdateThingsBoardIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::update_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::ThingsBoard,
            configuration: application::IntegrationConfiguration::ThingsBoard(
                application::ThingsBoardConfiguration {
                    server: req_int.server.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn delete_things_board_integration(
        &self,
        request: Request<api::DeleteThingsBoardIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        application::delete_integration(&app_id, application::IntegrationKind::ThingsBoard)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn create_my_devices_integration(
        &self,
        request: Request<api::CreateMyDevicesIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::create_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::MyDevices,
            configuration: application::IntegrationConfiguration::MyDevices(
                application::MyDevicesConfiguration {
                    endpoint: req_int.endpoint.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get_my_devices_integration(
        &self,
        request: Request<api::GetMyDevicesIntegrationRequest>,
    ) -> Result<Response<api::GetMyDevicesIntegrationResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let i = application::get_integration(&app_id, application::IntegrationKind::MyDevices)
            .await
            .map_err(|e| e.status())?;

        if let application::IntegrationConfiguration::MyDevices(conf) = &i.configuration {
            let mut resp = Response::new(api::GetMyDevicesIntegrationResponse {
                integration: Some(api::MyDevicesIntegration {
                    application_id: app_id.to_string(),
                    endpoint: conf.endpoint.clone(),
                }),
            });
            resp.metadata_mut()
                .insert("x-log-application_id", req.application_id.parse().unwrap());

            Ok(resp)
        } else {
            Err(Status::internal(
                "Integration has no MyDevices configuration",
            ))
        }
    }

    async fn update_my_devices_integration(
        &self,
        request: Request<api::UpdateMyDevicesIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::update_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::MyDevices,
            configuration: application::IntegrationConfiguration::MyDevices(
                application::MyDevicesConfiguration {
                    endpoint: req_int.endpoint.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn delete_my_devices_integration(
        &self,
        request: Request<api::DeleteMyDevicesIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        application::delete_integration(&app_id, application::IntegrationKind::MyDevices)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn create_lora_cloud_integration(
        &self,
        request: Request<api::CreateLoraCloudIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let req_mgs = match &req_int.modem_geolocation_services {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument(
                    "modem_geolocation_services configuration is missing",
                ));
            }
        };

        let _ = application::create_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::LoraCloud,
            configuration: application::IntegrationConfiguration::LoraCloud(
                application::LoraCloudConfiguration {
                    modem_geolocation_services: application::LoraCloudModemGeolocationServices {
                        token: req_mgs.token.clone(),
                        modem_enabled: req_mgs.modem_enabled,
                        forward_f_ports: req_mgs.forward_f_ports.clone(),
                        gnss_use_rx_time: req_mgs.gnss_use_rx_time,
                        gnss_use_gateway_location: req_mgs.gnss_use_gateway_location,
                        parse_tlv: req_mgs.parse_tlv,
                        geolocation_buffer_ttl: req_mgs.geolocation_buffer_ttl,
                        geolocation_min_buffer_size: req_mgs.geolocation_min_buffer_size,
                        geolocation_tdoa: req_mgs.geolocation_tdoa,
                        geolocation_rssi: req_mgs.geolocation_rssi,
                        geolocation_gnss: req_mgs.geolocation_gnss,
                        geolocation_gnss_payload_field: req_mgs
                            .geolocation_gnss_payload_field
                            .clone(),
                        geolocation_gnss_use_rx_time: req_mgs.geolocation_gnss_use_rx_time,
                        geolocation_wifi: req_mgs.geolocation_wifi,
                        geolocation_wifi_payload_field: req_mgs
                            .geolocation_wifi_payload_field
                            .clone(),
                        ..Default::default()
                    },
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get_lora_cloud_integration(
        &self,
        request: Request<api::GetLoraCloudIntegrationRequest>,
    ) -> Result<Response<api::GetLoraCloudIntegrationResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let i = application::get_integration(&app_id, application::IntegrationKind::LoraCloud)
            .await
            .map_err(|e| e.status())?;

        if let application::IntegrationConfiguration::LoraCloud(conf) = &i.configuration {
            let mgs = &conf.modem_geolocation_services;

            let mut resp = Response::new(api::GetLoraCloudIntegrationResponse {
                integration: Some(api::LoraCloudIntegration {
                    application_id: app_id.to_string(),
                    modem_geolocation_services: Some(api::LoraCloudModemGeolocationServices {
                        token: mgs.token.clone(),
                        modem_enabled: mgs.modem_enabled,
                        forward_f_ports: mgs.forward_f_ports.clone(),
                        gnss_use_rx_time: mgs.gnss_use_rx_time,
                        gnss_use_gateway_location: mgs.gnss_use_gateway_location,
                        parse_tlv: mgs.parse_tlv,
                        geolocation_buffer_ttl: mgs.geolocation_buffer_ttl,
                        geolocation_min_buffer_size: mgs.geolocation_min_buffer_size,
                        geolocation_tdoa: mgs.geolocation_tdoa,
                        geolocation_rssi: mgs.geolocation_rssi,
                        geolocation_gnss: mgs.geolocation_gnss,
                        geolocation_gnss_payload_field: mgs.geolocation_gnss_payload_field.clone(),
                        geolocation_gnss_use_rx_time: mgs.geolocation_gnss_use_rx_time,
                        geolocation_wifi: mgs.geolocation_wifi,
                        geolocation_wifi_payload_field: mgs.geolocation_wifi_payload_field.clone(),
                    }),
                }),
            });
            resp.metadata_mut()
                .insert("x-log-application_id", req.application_id.parse().unwrap());

            Ok(resp)
        } else {
            Err(Status::internal(
                "Integration has no LoraCloud configuration",
            ))
        }
    }

    async fn update_lora_cloud_integration(
        &self,
        request: Request<api::UpdateLoraCloudIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let req_mgs = match &req_int.modem_geolocation_services {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument(
                    "modem_geolocation_services configuration is missing",
                ));
            }
        };

        let _ = application::update_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::LoraCloud,
            configuration: application::IntegrationConfiguration::LoraCloud(
                application::LoraCloudConfiguration {
                    modem_geolocation_services: application::LoraCloudModemGeolocationServices {
                        token: req_mgs.token.clone(),
                        modem_enabled: req_mgs.modem_enabled,
                        forward_f_ports: req_mgs.forward_f_ports.clone(),
                        gnss_use_rx_time: req_mgs.gnss_use_rx_time,
                        gnss_use_gateway_location: req_mgs.gnss_use_gateway_location,
                        parse_tlv: req_mgs.parse_tlv,
                        geolocation_buffer_ttl: req_mgs.geolocation_buffer_ttl,
                        geolocation_min_buffer_size: req_mgs.geolocation_min_buffer_size,
                        geolocation_tdoa: req_mgs.geolocation_tdoa,
                        geolocation_rssi: req_mgs.geolocation_rssi,
                        geolocation_gnss: req_mgs.geolocation_gnss,
                        geolocation_gnss_payload_field: req_mgs
                            .geolocation_gnss_payload_field
                            .clone(),
                        geolocation_gnss_use_rx_time: req_mgs.geolocation_gnss_use_rx_time,
                        geolocation_wifi: req_mgs.geolocation_wifi,
                        geolocation_wifi_payload_field: req_mgs
                            .geolocation_wifi_payload_field
                            .clone(),
                        ..Default::default()
                    },
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn delete_lora_cloud_integration(
        &self,
        request: Request<api::DeleteLoraCloudIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        application::delete_integration(&app_id, application::IntegrationKind::LoraCloud)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn create_gcp_pub_sub_integration(
        &self,
        request: Request<api::CreateGcpPubSubIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::create_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::GcpPubSub,
            configuration: application::IntegrationConfiguration::GcpPubSub(
                application::GcpPubSubConfiguration {
                    encoding: req_int.encoding,
                    credentials_file: req_int.credentials_file.clone(),
                    project_id: req_int.project_id.clone(),
                    topic_name: req_int.topic_name.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get_gcp_pub_sub_integration(
        &self,
        request: Request<api::GetGcpPubSubIntegrationRequest>,
    ) -> Result<Response<api::GetGcpPubSubIntegrationResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let i = application::get_integration(&app_id, application::IntegrationKind::GcpPubSub)
            .await
            .map_err(|e| e.status())?;

        if let application::IntegrationConfiguration::GcpPubSub(conf) = &i.configuration {
            let mut resp = Response::new(api::GetGcpPubSubIntegrationResponse {
                integration: Some(api::GcpPubSubIntegration {
                    application_id: app_id.to_string(),
                    encoding: conf.encoding,
                    credentials_file: conf.credentials_file.clone(),
                    project_id: conf.project_id.clone(),
                    topic_name: conf.topic_name.clone(),
                }),
            });
            resp.metadata_mut()
                .insert("x-log-application_id", req.application_id.parse().unwrap());

            Ok(resp)
        } else {
            Err(Status::internal(
                "Integration has no GcpPubSub configuration",
            ))
        }
    }

    async fn update_gcp_pub_sub_integration(
        &self,
        request: Request<api::UpdateGcpPubSubIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::update_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::GcpPubSub,
            configuration: application::IntegrationConfiguration::GcpPubSub(
                application::GcpPubSubConfiguration {
                    encoding: req_int.encoding,
                    credentials_file: req_int.credentials_file.clone(),
                    project_id: req_int.project_id.clone(),
                    topic_name: req_int.topic_name.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn delete_gcp_pub_sub_integration(
        &self,
        request: Request<api::DeleteGcpPubSubIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        application::delete_integration(&app_id, application::IntegrationKind::GcpPubSub)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn create_aws_sns_integration(
        &self,
        request: Request<api::CreateAwsSnsIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::create_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::AwsSns,
            configuration: application::IntegrationConfiguration::AwsSns(
                application::AwsSnsConfiguration {
                    encoding: req_int.encoding,
                    region: req_int.region.clone(),
                    access_key_id: req_int.access_key_id.clone(),
                    secret_access_key: req_int.secret_access_key.clone(),
                    topic_arn: req_int.topic_arn.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get_aws_sns_integration(
        &self,
        request: Request<api::GetAwsSnsIntegrationRequest>,
    ) -> Result<Response<api::GetAwsSnsIntegrationResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let i = application::get_integration(&app_id, application::IntegrationKind::AwsSns)
            .await
            .map_err(|e| e.status())?;

        if let application::IntegrationConfiguration::AwsSns(conf) = &i.configuration {
            let mut resp = Response::new(api::GetAwsSnsIntegrationResponse {
                integration: Some(api::AwsSnsIntegration {
                    application_id: app_id.to_string(),
                    encoding: conf.encoding,
                    region: conf.region.clone(),
                    access_key_id: conf.access_key_id.clone(),
                    secret_access_key: conf.secret_access_key.clone(),
                    topic_arn: conf.topic_arn.clone(),
                }),
            });
            resp.metadata_mut()
                .insert("x-log-application_id", req.application_id.parse().unwrap());

            Ok(resp)
        } else {
            Err(Status::internal("Integration has no AwsSns configuration"))
        }
    }

    async fn update_aws_sns_integration(
        &self,
        request: Request<api::UpdateAwsSnsIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::update_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::AwsSns,
            configuration: application::IntegrationConfiguration::AwsSns(
                application::AwsSnsConfiguration {
                    encoding: req_int.encoding,
                    region: req_int.region.clone(),
                    access_key_id: req_int.access_key_id.clone(),
                    secret_access_key: req_int.secret_access_key.clone(),
                    topic_arn: req_int.topic_arn.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn delete_aws_sns_integration(
        &self,
        request: Request<api::DeleteAwsSnsIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        application::delete_integration(&app_id, application::IntegrationKind::AwsSns)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn create_azure_service_bus_integration(
        &self,
        request: Request<api::CreateAzureServiceBusIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::create_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::AzureServiceBus,
            configuration: application::IntegrationConfiguration::AzureServiceBus(
                application::AzureServiceBusConfiguration {
                    encoding: req_int.encoding,
                    connection_string: req_int.connection_string.clone(),
                    publish_name: req_int.publish_name.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get_azure_service_bus_integration(
        &self,
        request: Request<api::GetAzureServiceBusIntegrationRequest>,
    ) -> Result<Response<api::GetAzureServiceBusIntegrationResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let i =
            application::get_integration(&app_id, application::IntegrationKind::AzureServiceBus)
                .await
                .map_err(|e| e.status())?;

        if let application::IntegrationConfiguration::AzureServiceBus(conf) = &i.configuration {
            let mut resp = Response::new(api::GetAzureServiceBusIntegrationResponse {
                integration: Some(api::AzureServiceBusIntegration {
                    application_id: app_id.to_string(),
                    encoding: conf.encoding,
                    connection_string: conf.connection_string.clone(),
                    publish_name: conf.publish_name.clone(),
                }),
            });
            resp.metadata_mut()
                .insert("x-log-application_id", req.application_id.parse().unwrap());

            Ok(resp)
        } else {
            Err(Status::internal(
                "Integration has no AzureServiceBus configuration",
            ))
        }
    }

    async fn update_azure_service_bus_integration(
        &self,
        request: Request<api::UpdateAzureServiceBusIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::update_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::AzureServiceBus,
            configuration: application::IntegrationConfiguration::AzureServiceBus(
                application::AzureServiceBusConfiguration {
                    encoding: req_int.encoding,
                    connection_string: req_int.connection_string.clone(),
                    publish_name: req_int.publish_name.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn delete_azure_service_bus_integration(
        &self,
        request: Request<api::DeleteAzureServiceBusIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        application::delete_integration(&app_id, application::IntegrationKind::AzureServiceBus)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn create_pilot_things_integration(
        &self,
        request: Request<api::CreatePilotThingsIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::create_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::PilotThings,
            configuration: application::IntegrationConfiguration::PilotThings(
                application::PilotThingsConfiguration {
                    server: req_int.server.clone(),
                    token: req_int.token.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get_pilot_things_integration(
        &self,
        request: Request<api::GetPilotThingsIntegrationRequest>,
    ) -> Result<Response<api::GetPilotThingsIntegrationResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let i = application::get_integration(&app_id, application::IntegrationKind::PilotThings)
            .await
            .map_err(|e| e.status())?;

        if let application::IntegrationConfiguration::PilotThings(conf) = &i.configuration {
            let mut resp = Response::new(api::GetPilotThingsIntegrationResponse {
                integration: Some(api::PilotThingsIntegration {
                    application_id: app_id.to_string(),
                    server: conf.server.clone(),
                    token: conf.token.clone(),
                }),
            });
            resp.metadata_mut()
                .insert("x-log-application_id", req.application_id.parse().unwrap());

            Ok(resp)
        } else {
            Err(Status::internal(
                "Integration has no PilotThings configuration",
            ))
        }
    }

    async fn update_pilot_things_integration(
        &self,
        request: Request<api::UpdatePilotThingsIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::update_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::PilotThings,
            configuration: application::IntegrationConfiguration::PilotThings(
                application::PilotThingsConfiguration {
                    server: req_int.server.clone(),
                    token: req_int.token.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn delete_pilot_things_integration(
        &self,
        request: Request<api::DeletePilotThingsIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        application::delete_integration(&app_id, application::IntegrationKind::PilotThings)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn create_ifttt_integration(
        &self,
        request: Request<api::CreateIftttIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        if !req_int.event_prefix.is_empty()
            && !regex::Regex::new(r"^[a-zA-Z0-9]+$")
                .unwrap()
                .is_match(&req_int.event_prefix)
        {
            return Err(Status::invalid_argument(
                "event_prefix may only contain A-Z, a-z and 0-9 characters",
            ));
        }

        let _ = application::create_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::Ifttt,
            configuration: application::IntegrationConfiguration::Ifttt(
                application::IftttConfiguration {
                    key: req_int.key.clone(),
                    uplink_values: [
                        req_int.uplink_values.get(0).cloned().unwrap_or_default(),
                        req_int.uplink_values.get(1).cloned().unwrap_or_default(),
                    ],
                    arbitrary_json: req_int.arbitrary_json,
                    event_prefix: req_int.event_prefix.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get_ifttt_integration(
        &self,
        request: Request<api::GetIftttIntegrationRequest>,
    ) -> Result<Response<api::GetIftttIntegrationResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        let i = application::get_integration(&app_id, application::IntegrationKind::Ifttt)
            .await
            .map_err(|e| e.status())?;

        if let application::IntegrationConfiguration::Ifttt(conf) = &i.configuration {
            let mut resp = Response::new(api::GetIftttIntegrationResponse {
                integration: Some(api::IftttIntegration {
                    application_id: app_id.to_string(),
                    key: conf.key.clone(),
                    uplink_values: conf.uplink_values.to_vec(),
                    arbitrary_json: conf.arbitrary_json,
                    event_prefix: conf.event_prefix.clone(),
                }),
            });
            resp.metadata_mut()
                .insert("x-log-application_id", req.application_id.parse().unwrap());

            Ok(resp)
        } else {
            Err(Status::internal("Integration has no Ifttt configuration"))
        }
    }

    async fn update_ifttt_integration(
        &self,
        request: Request<api::UpdateIftttIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req_int = match &request.get_ref().integration {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("integration is missing"));
            }
        };
        let app_id = Uuid::from_str(&req_int.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let _ = application::update_integration(application::Integration {
            application_id: app_id,
            kind: application::IntegrationKind::Ifttt,
            configuration: application::IntegrationConfiguration::Ifttt(
                application::IftttConfiguration {
                    key: req_int.key.clone(),
                    uplink_values: [
                        req_int.uplink_values.get(0).cloned().unwrap_or_default(),
                        req_int.uplink_values.get(1).cloned().unwrap_or_default(),
                    ],
                    arbitrary_json: req_int.arbitrary_json,
                    event_prefix: req_int.event_prefix.clone(),
                },
            ),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-application_id",
            req_int.application_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn delete_ifttt_integration(
        &self,
        request: Request<api::DeleteIftttIntegrationRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        application::delete_integration(&app_id, application::IntegrationKind::Ifttt)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn generate_mqtt_integration_client_certificate(
        &self,
        request: Request<api::GenerateMqttIntegrationClientCertificateRequest>,
    ) -> Result<Response<api::GenerateMqttIntegrationClientCertificateResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Update, app_id),
            )
            .await?;

        let (ttl, ca_cert, cert, key) = certificate::client_cert_for_application_id(&app_id)
            .await
            .map_err(|e| e.status())?;

        application::update_mqtt_cls_cert(&app_id, cert.as_bytes())
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::GenerateMqttIntegrationClientCertificateResponse {
            ca_cert,
            tls_cert: cert,
            tls_key: key,
            expires_at: Some(ttl.into()),
        });
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::api::auth::validator::RequestValidator;
    use crate::api::auth::AuthID;
    use crate::storage::{tenant, user};
    use crate::test;

    #[tokio::test]
    async fn test_application() {
        let _guard = test::prepare().await;

        // setup admin user
        let u = user::User {
            is_admin: true,
            is_active: true,
            email: "admin@admin".into(),
            email_verified: true,
            ..Default::default()
        };
        let u = user::create(u).await.unwrap();

        // create tenant
        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            can_have_gateways: true,
            max_gateway_count: 10,
            ..Default::default()
        })
        .await
        .unwrap();

        // setup api
        let service = Application::new(RequestValidator::new());

        // create
        let create_req = api::CreateApplicationRequest {
            application: Some(api::Application {
                tenant_id: t.id.to_string(),
                name: "test-app".into(),
                ..Default::default()
            }),
        };
        let mut create_req = Request::new(create_req);
        create_req
            .extensions_mut()
            .insert(AuthID::User(u.id.clone()));
        let create_resp = service.create(create_req).await.unwrap();
        let create_resp = create_resp.get_ref();

        //get
        let get_req = api::GetApplicationRequest {
            id: create_resp.id.clone(),
        };
        let mut get_req = Request::new(get_req);
        get_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::Application {
                id: create_resp.id.clone(),
                tenant_id: t.id.to_string(),
                name: "test-app".into(),
                ..Default::default()
            }),
            get_resp.get_ref().application
        );

        // update
        let up_req = api::UpdateApplicationRequest {
            application: Some(api::Application {
                id: create_resp.id.clone(),
                tenant_id: t.id.to_string(),
                name: "updated-app".into(),
                ..Default::default()
            }),
        };
        let mut up_req = Request::new(up_req);
        up_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let _ = service.update(up_req).await.unwrap();

        //get
        let get_req = api::GetApplicationRequest {
            id: create_resp.id.clone(),
        };
        let mut get_req = Request::new(get_req);
        get_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::Application {
                id: create_resp.id.clone(),
                tenant_id: t.id.to_string(),
                name: "updated-app".into(),
                ..Default::default()
            }),
            get_resp.get_ref().application
        );

        // list
        let list_req = api::ListApplicationsRequest {
            search: "updated".into(),
            tenant_id: t.id.to_string(),
            limit: 10,
            offset: 0,
        };
        let mut list_req = Request::new(list_req);
        list_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let list_resp = service.list(list_req).await.unwrap();
        assert_eq!(1, list_resp.get_ref().total_count);
        assert_eq!(1, list_resp.get_ref().result.len());

        // delete
        let del_req = api::DeleteApplicationRequest {
            id: create_resp.id.clone(),
        };
        let mut del_req = Request::new(del_req);
        del_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let _ = service.delete(del_req).await.unwrap();

        let del_req = api::DeleteApplicationRequest {
            id: create_resp.id.clone(),
        };
        let mut del_req = Request::new(del_req);
        del_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let del_resp = service.delete(del_req).await;
        assert!(del_resp.is_err());
    }

    async fn get_application() -> application::Application {
        // create tenant
        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        application::create(application::Application {
            tenant_id: t.id.clone(),
            name: "test-app".into(),
            ..Default::default()
        })
        .await
        .unwrap()
    }

    async fn get_user() -> user::User {
        let u = user::User {
            is_admin: true,
            is_active: true,
            email: "admin@admin".into(),
            email_verified: true,
            ..Default::default()
        };
        user::create(u).await.unwrap()
    }

    fn get_request<T>(user_id: &Uuid, req: T) -> Request<T> {
        let mut req = Request::new(req);
        req.extensions_mut().insert(AuthID::User(user_id.clone()));
        req
    }

    #[tokio::test]
    async fn test_http_integration() {
        let _guard = test::prepare().await;
        let app = get_application().await;
        let u = get_user().await;
        let service = Application::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateHttpIntegrationRequest {
                integration: Some(api::HttpIntegration {
                    application_id: app.id.to_string(),
                    headers: [("Foo".to_string(), "Bar".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    encoding: api::Encoding::Json.into(),
                    event_endpoint_url: "http://example.com".into(),
                }),
            },
        );
        let _ = service.create_http_integration(create_req).await.unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetHttpIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_http_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::HttpIntegration {
                application_id: app.id.to_string(),
                headers: [("Foo".to_string(), "Bar".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
                encoding: api::Encoding::Json.into(),
                event_endpoint_url: "http://example.com".into(),
            }),
            get_resp.integration
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateHttpIntegrationRequest {
                integration: Some(api::HttpIntegration {
                    application_id: app.id.to_string(),
                    headers: [("Foo".to_string(), "Bas".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    encoding: api::Encoding::Protobuf.into(),
                    event_endpoint_url: "http://example.org".into(),
                }),
            },
        );
        let _ = service.update_http_integration(update_req).await.unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetHttpIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_http_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::HttpIntegration {
                application_id: app.id.to_string(),
                headers: [("Foo".to_string(), "Bas".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
                encoding: api::Encoding::Protobuf.into(),
                event_endpoint_url: "http://example.org".into(),
            }),
            get_resp.integration
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 2,
                result: vec![
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::Http.into(),
                    },
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::MqttGlobal.into(),
                    }
                ],
            },
            list_resp
        );

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteHttpIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let _ = service.delete_http_integration(del_req).await.unwrap();

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 1,
                result: vec![api::IntegrationListItem {
                    kind: api::IntegrationKind::MqttGlobal.into(),
                },],
            },
            list_resp
        );
    }

    #[tokio::test]
    async fn test_influx_db_integration() {
        let _guard = test::prepare().await;
        let app = get_application().await;
        let u = get_user().await;
        let service = Application::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateInfluxDbIntegrationRequest {
                integration: Some(api::InfluxDbIntegration {
                    application_id: app.id.to_string(),
                    endpoint: "http://influxdb/".into(),
                    db: "testdb".into(),
                    username: "testuser".into(),
                    password: "testpw".into(),
                    retention_policy_name: "DEFAULT".into(),
                    precision: api::InfluxDbPrecision::S.into(),
                    version: api::InfluxDbVersion::Influxdb1.into(),
                    token: "testtoken".into(),
                    organization: "testorg".into(),
                    bucket: "testbucket".into(),
                }),
            },
        );
        let _ = service
            .create_influx_db_integration(create_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetInfluxDbIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_influx_db_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::InfluxDbIntegration {
                application_id: app.id.to_string(),
                endpoint: "http://influxdb/".into(),
                db: "testdb".into(),
                username: "testuser".into(),
                password: "testpw".into(),
                retention_policy_name: "DEFAULT".into(),
                precision: api::InfluxDbPrecision::S.into(),
                version: api::InfluxDbVersion::Influxdb1.into(),
                token: "testtoken".into(),
                organization: "testorg".into(),
                bucket: "testbucket".into(),
            }),
            get_resp.integration
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateInfluxDbIntegrationRequest {
                integration: Some(api::InfluxDbIntegration {
                    application_id: app.id.to_string(),
                    endpoint: "http://influxdb-updated/".into(),
                    db: "testdb".into(),
                    username: "testuser".into(),
                    password: "testpw".into(),
                    retention_policy_name: "DEFAULT".into(),
                    precision: api::InfluxDbPrecision::S.into(),
                    version: api::InfluxDbVersion::Influxdb1.into(),
                    token: "testtoken".into(),
                    organization: "testorg".into(),
                    bucket: "testbucket".into(),
                }),
            },
        );
        let _ = service
            .update_influx_db_integration(update_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetInfluxDbIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_influx_db_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::InfluxDbIntegration {
                application_id: app.id.to_string(),
                endpoint: "http://influxdb-updated/".into(),
                db: "testdb".into(),
                username: "testuser".into(),
                password: "testpw".into(),
                retention_policy_name: "DEFAULT".into(),
                precision: api::InfluxDbPrecision::S.into(),
                version: api::InfluxDbVersion::Influxdb1.into(),
                token: "testtoken".into(),
                organization: "testorg".into(),
                bucket: "testbucket".into(),
            }),
            get_resp.integration
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 2,
                result: vec![
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::InfluxDb.into(),
                    },
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::MqttGlobal.into(),
                    }
                ],
            },
            list_resp
        );

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteInfluxDbIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let _ = service.delete_influx_db_integration(del_req).await.unwrap();

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 1,
                result: vec![api::IntegrationListItem {
                    kind: api::IntegrationKind::MqttGlobal.into(),
                },],
            },
            list_resp
        );
    }

    #[tokio::test]
    async fn test_things_board_integration() {
        let _guard = test::prepare().await;
        let app = get_application().await;
        let u = get_user().await;
        let service = Application::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateThingsBoardIntegrationRequest {
                integration: Some(api::ThingsBoardIntegration {
                    application_id: app.id.to_string(),
                    server: "http://thingsboard/".into(),
                }),
            },
        );
        let _ = service
            .create_things_board_integration(create_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetThingsBoardIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_things_board_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::ThingsBoardIntegration {
                application_id: app.id.to_string(),
                server: "http://thingsboard/".into(),
            }),
            get_resp.integration
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateThingsBoardIntegrationRequest {
                integration: Some(api::ThingsBoardIntegration {
                    application_id: app.id.to_string(),
                    server: "http://thingsboard-updated/".into(),
                }),
            },
        );
        let _ = service
            .update_things_board_integration(update_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetThingsBoardIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_things_board_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::ThingsBoardIntegration {
                application_id: app.id.to_string(),
                server: "http://thingsboard-updated/".into(),
            }),
            get_resp.integration
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 2,
                result: vec![
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::ThingsBoard.into(),
                    },
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::MqttGlobal.into(),
                    }
                ],
            },
            list_resp
        );

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteThingsBoardIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let _ = service
            .delete_things_board_integration(del_req)
            .await
            .unwrap();

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 1,
                result: vec![api::IntegrationListItem {
                    kind: api::IntegrationKind::MqttGlobal.into(),
                },],
            },
            list_resp
        );
    }

    #[tokio::test]
    async fn test_my_devices_integration() {
        let _guard = test::prepare().await;
        let app = get_application().await;
        let u = get_user().await;
        let service = Application::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateMyDevicesIntegrationRequest {
                integration: Some(api::MyDevicesIntegration {
                    application_id: app.id.to_string(),
                    endpoint: "http://mydevices".into(),
                }),
            },
        );
        let _ = service
            .create_my_devices_integration(create_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetMyDevicesIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_my_devices_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::MyDevicesIntegration {
                application_id: app.id.to_string(),
                endpoint: "http://mydevices".into(),
            }),
            get_resp.integration
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateMyDevicesIntegrationRequest {
                integration: Some(api::MyDevicesIntegration {
                    application_id: app.id.to_string(),
                    endpoint: "http://mydevices-updated".into(),
                }),
            },
        );
        let _ = service
            .update_my_devices_integration(update_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetMyDevicesIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_my_devices_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::MyDevicesIntegration {
                application_id: app.id.to_string(),
                endpoint: "http://mydevices-updated".into(),
            }),
            get_resp.integration
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 2,
                result: vec![
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::MyDevices.into(),
                    },
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::MqttGlobal.into(),
                    }
                ],
            },
            list_resp
        );

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteMyDevicesIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let _ = service
            .delete_my_devices_integration(del_req)
            .await
            .unwrap();

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 1,
                result: vec![api::IntegrationListItem {
                    kind: api::IntegrationKind::MqttGlobal.into(),
                },],
            },
            list_resp
        );
    }

    #[tokio::test]
    async fn test_lora_cloud_integration() {
        let _guard = test::prepare().await;
        let app = get_application().await;
        let u = get_user().await;
        let service = Application::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateLoraCloudIntegrationRequest {
                integration: Some(api::LoraCloudIntegration {
                    application_id: app.id.to_string(),
                    modem_geolocation_services: Some(api::LoraCloudModemGeolocationServices {
                        token: "test-token".into(),
                        modem_enabled: true,
                        forward_f_ports: vec![199, 198, 197, 192],
                        gnss_use_rx_time: true,
                        gnss_use_gateway_location: true,
                        parse_tlv: true,
                        geolocation_buffer_ttl: 300,
                        geolocation_min_buffer_size: 2,
                        geolocation_tdoa: true,
                        geolocation_rssi: true,
                        geolocation_gnss: true,
                        geolocation_gnss_payload_field: "gnss_pl".into(),
                        geolocation_gnss_use_rx_time: true,
                        geolocation_wifi: true,
                        geolocation_wifi_payload_field: "wifi_pl".into(),
                    }),
                }),
            },
        );
        let _ = service
            .create_lora_cloud_integration(create_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetLoraCloudIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_lora_cloud_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::LoraCloudIntegration {
                application_id: app.id.to_string(),
                modem_geolocation_services: Some(api::LoraCloudModemGeolocationServices {
                    token: "test-token".into(),
                    modem_enabled: true,
                    forward_f_ports: vec![199, 198, 197, 192],
                    gnss_use_rx_time: true,
                    gnss_use_gateway_location: true,
                    parse_tlv: true,
                    geolocation_buffer_ttl: 300,
                    geolocation_min_buffer_size: 2,
                    geolocation_tdoa: true,
                    geolocation_rssi: true,
                    geolocation_gnss: true,
                    geolocation_gnss_payload_field: "gnss_pl".into(),
                    geolocation_gnss_use_rx_time: true,
                    geolocation_wifi: true,
                    geolocation_wifi_payload_field: "wifi_pl".into(),
                }),
            }),
            get_resp.integration
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateLoraCloudIntegrationRequest {
                integration: Some(api::LoraCloudIntegration {
                    application_id: app.id.to_string(),
                    modem_geolocation_services: Some(api::LoraCloudModemGeolocationServices {
                        token: "test-token-updated".into(),
                        modem_enabled: true,
                        forward_f_ports: vec![199, 198, 197, 192],
                        gnss_use_rx_time: true,
                        gnss_use_gateway_location: true,
                        parse_tlv: true,
                        geolocation_buffer_ttl: 300,
                        geolocation_min_buffer_size: 2,
                        geolocation_tdoa: true,
                        geolocation_rssi: true,
                        geolocation_gnss: true,
                        geolocation_gnss_payload_field: "gnss_pl".into(),
                        geolocation_gnss_use_rx_time: true,
                        geolocation_wifi: true,
                        geolocation_wifi_payload_field: "wifi_pl".into(),
                    }),
                }),
            },
        );
        let _ = service
            .update_lora_cloud_integration(update_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetLoraCloudIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_lora_cloud_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::LoraCloudIntegration {
                application_id: app.id.to_string(),
                modem_geolocation_services: Some(api::LoraCloudModemGeolocationServices {
                    token: "test-token-updated".into(),
                    modem_enabled: true,
                    forward_f_ports: vec![199, 198, 197, 192],
                    gnss_use_rx_time: true,
                    gnss_use_gateway_location: true,
                    parse_tlv: true,
                    geolocation_buffer_ttl: 300,
                    geolocation_min_buffer_size: 2,
                    geolocation_tdoa: true,
                    geolocation_rssi: true,
                    geolocation_gnss: true,
                    geolocation_gnss_payload_field: "gnss_pl".into(),
                    geolocation_gnss_use_rx_time: true,
                    geolocation_wifi: true,
                    geolocation_wifi_payload_field: "wifi_pl".into(),
                }),
            }),
            get_resp.integration
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 2,
                result: vec![
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::LoraCloud.into(),
                    },
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::MqttGlobal.into(),
                    }
                ],
            },
            list_resp
        );

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteLoraCloudIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let _ = service
            .delete_lora_cloud_integration(del_req)
            .await
            .unwrap();

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 1,
                result: vec![api::IntegrationListItem {
                    kind: api::IntegrationKind::MqttGlobal.into(),
                },],
            },
            list_resp
        );
    }

    #[tokio::test]
    async fn test_gcp_pub_sub_integration() {
        let _guard = test::prepare().await;
        let app = get_application().await;
        let u = get_user().await;
        let service = Application::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateGcpPubSubIntegrationRequest {
                integration: Some(api::GcpPubSubIntegration {
                    application_id: app.id.to_string(),
                    encoding: api::Encoding::Json.into(),
                    credentials_file: "--credentials--".into(),
                    project_id: "test-project".into(),
                    topic_name: "test-topic".into(),
                }),
            },
        );
        let _ = service
            .create_gcp_pub_sub_integration(create_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetGcpPubSubIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_gcp_pub_sub_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::GcpPubSubIntegration {
                application_id: app.id.to_string(),
                encoding: api::Encoding::Json.into(),
                credentials_file: "--credentials--".into(),
                project_id: "test-project".into(),
                topic_name: "test-topic".into(),
            }),
            get_resp.integration
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateGcpPubSubIntegrationRequest {
                integration: Some(api::GcpPubSubIntegration {
                    application_id: app.id.to_string(),
                    encoding: api::Encoding::Protobuf.into(),
                    credentials_file: "--credentials--".into(),
                    project_id: "test-project".into(),
                    topic_name: "test-topic".into(),
                }),
            },
        );
        let _ = service
            .update_gcp_pub_sub_integration(update_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetGcpPubSubIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_gcp_pub_sub_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::GcpPubSubIntegration {
                application_id: app.id.to_string(),
                encoding: api::Encoding::Protobuf.into(),
                credentials_file: "--credentials--".into(),
                project_id: "test-project".into(),
                topic_name: "test-topic".into(),
            }),
            get_resp.integration
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 2,
                result: vec![
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::GcpPubSub.into(),
                    },
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::MqttGlobal.into(),
                    }
                ],
            },
            list_resp
        );

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteGcpPubSubIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let _ = service
            .delete_gcp_pub_sub_integration(del_req)
            .await
            .unwrap();

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 1,
                result: vec![api::IntegrationListItem {
                    kind: api::IntegrationKind::MqttGlobal.into(),
                },],
            },
            list_resp
        );
    }

    #[tokio::test]
    async fn test_aws_sns_integration() {
        let _guard = test::prepare().await;
        let app = get_application().await;
        let u = get_user().await;
        let service = Application::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateAwsSnsIntegrationRequest {
                integration: Some(api::AwsSnsIntegration {
                    application_id: app.id.to_string(),
                    encoding: api::Encoding::Json.into(),
                    region: "eu-west1".into(),
                    access_key_id: "keyid".into(),
                    secret_access_key: "secretkey".into(),
                    topic_arn: "topicarn".into(),
                }),
            },
        );
        let _ = service
            .create_aws_sns_integration(create_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetAwsSnsIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_aws_sns_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::AwsSnsIntegration {
                application_id: app.id.to_string(),
                encoding: api::Encoding::Json.into(),
                region: "eu-west1".into(),
                access_key_id: "keyid".into(),
                secret_access_key: "secretkey".into(),
                topic_arn: "topicarn".into(),
            }),
            get_resp.integration
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateAwsSnsIntegrationRequest {
                integration: Some(api::AwsSnsIntegration {
                    application_id: app.id.to_string(),
                    encoding: api::Encoding::Protobuf.into(),
                    region: "eu-west1".into(),
                    access_key_id: "keyid".into(),
                    secret_access_key: "secretkey".into(),
                    topic_arn: "topicarn".into(),
                }),
            },
        );
        let _ = service
            .update_aws_sns_integration(update_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetAwsSnsIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_aws_sns_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::AwsSnsIntegration {
                application_id: app.id.to_string(),
                encoding: api::Encoding::Protobuf.into(),
                region: "eu-west1".into(),
                access_key_id: "keyid".into(),
                secret_access_key: "secretkey".into(),
                topic_arn: "topicarn".into(),
            }),
            get_resp.integration
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 2,
                result: vec![
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::AwsSns.into(),
                    },
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::MqttGlobal.into(),
                    }
                ],
            },
            list_resp
        );

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteAwsSnsIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let _ = service.delete_aws_sns_integration(del_req).await.unwrap();

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 1,
                result: vec![api::IntegrationListItem {
                    kind: api::IntegrationKind::MqttGlobal.into(),
                },],
            },
            list_resp
        );
    }

    #[tokio::test]
    async fn test_azure_service_bus_integration() {
        let _guard = test::prepare().await;
        let app = get_application().await;
        let u = get_user().await;
        let service = Application::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateAzureServiceBusIntegrationRequest {
                integration: Some(api::AzureServiceBusIntegration {
                    application_id: app.id.to_string(),
                    encoding: api::Encoding::Json.into(),
                    connection_string: "connection-string".into(),
                    publish_name: "publish-name".into(),
                }),
            },
        );
        let _ = service
            .create_azure_service_bus_integration(create_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetAzureServiceBusIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service
            .get_azure_service_bus_integration(get_req)
            .await
            .unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::AzureServiceBusIntegration {
                application_id: app.id.to_string(),
                encoding: api::Encoding::Json.into(),
                connection_string: "connection-string".into(),
                publish_name: "publish-name".into(),
            }),
            get_resp.integration
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateAzureServiceBusIntegrationRequest {
                integration: Some(api::AzureServiceBusIntegration {
                    application_id: app.id.to_string(),
                    encoding: api::Encoding::Protobuf.into(),
                    connection_string: "connection-string".into(),
                    publish_name: "publish-name".into(),
                }),
            },
        );
        let _ = service
            .update_azure_service_bus_integration(update_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetAzureServiceBusIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service
            .get_azure_service_bus_integration(get_req)
            .await
            .unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::AzureServiceBusIntegration {
                application_id: app.id.to_string(),
                encoding: api::Encoding::Protobuf.into(),
                connection_string: "connection-string".into(),
                publish_name: "publish-name".into(),
            }),
            get_resp.integration
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 2,
                result: vec![
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::AzureServiceBus.into(),
                    },
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::MqttGlobal.into(),
                    }
                ],
            },
            list_resp
        );

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteAzureServiceBusIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let _ = service
            .delete_azure_service_bus_integration(del_req)
            .await
            .unwrap();

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 1,
                result: vec![api::IntegrationListItem {
                    kind: api::IntegrationKind::MqttGlobal.into(),
                },],
            },
            list_resp
        );
    }

    #[tokio::test]
    async fn test_pilot_things_integration() {
        let _guard = test::prepare().await;
        let app = get_application().await;
        let u = get_user().await;
        let service = Application::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreatePilotThingsIntegrationRequest {
                integration: Some(api::PilotThingsIntegration {
                    application_id: app.id.to_string(),
                    server: "http://pilotthings".into(),
                    token: "secrettoken".into(),
                }),
            },
        );
        let _ = service
            .create_pilot_things_integration(create_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetPilotThingsIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_pilot_things_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::PilotThingsIntegration {
                application_id: app.id.to_string(),
                server: "http://pilotthings".into(),
                token: "secrettoken".into(),
            }),
            get_resp.integration
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdatePilotThingsIntegrationRequest {
                integration: Some(api::PilotThingsIntegration {
                    application_id: app.id.to_string(),
                    server: "http://pilotthings-updated".into(),
                    token: "secrettoken".into(),
                }),
            },
        );
        let _ = service
            .update_pilot_things_integration(update_req)
            .await
            .unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetPilotThingsIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_pilot_things_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::PilotThingsIntegration {
                application_id: app.id.to_string(),
                server: "http://pilotthings-updated".into(),
                token: "secrettoken".into(),
            }),
            get_resp.integration
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 2,
                result: vec![
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::PilotThings.into(),
                    },
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::MqttGlobal.into(),
                    }
                ],
            },
            list_resp
        );

        // delete
        let del_req = get_request(
            &u.id,
            api::DeletePilotThingsIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let _ = service
            .delete_pilot_things_integration(del_req)
            .await
            .unwrap();

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 1,
                result: vec![api::IntegrationListItem {
                    kind: api::IntegrationKind::MqttGlobal.into(),
                },],
            },
            list_resp
        );
    }

    #[tokio::test]
    async fn test_ifttt_integration() {
        let _guard = test::prepare().await;
        let app = get_application().await;
        let u = get_user().await;
        let service = Application::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateIftttIntegrationRequest {
                integration: Some(api::IftttIntegration {
                    application_id: app.id.to_string(),
                    key: "verysecret".into(),
                    uplink_values: vec!["value_1".into(), "value_2".into()],
                    arbitrary_json: false,
                    event_prefix: "foo".to_string(),
                }),
            },
        );
        let _ = service.create_ifttt_integration(create_req).await.unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetIftttIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_ifttt_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::IftttIntegration {
                application_id: app.id.to_string(),
                key: "verysecret".into(),
                uplink_values: vec!["value_1".into(), "value_2".into()],
                arbitrary_json: false,
                event_prefix: "foo".to_string(),
            }),
            get_resp.integration
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateIftttIntegrationRequest {
                integration: Some(api::IftttIntegration {
                    application_id: app.id.to_string(),
                    key: "verysecrettoo".into(),
                    uplink_values: vec!["value_4".into(), "value_5".into()],
                    arbitrary_json: true,
                    event_prefix: "bar".to_string(),
                }),
            },
        );
        let _ = service.update_ifttt_integration(update_req).await.unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetIftttIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let get_resp = service.get_ifttt_integration(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::IftttIntegration {
                application_id: app.id.to_string(),
                key: "verysecrettoo".into(),
                uplink_values: vec!["value_4".into(), "value_5".into()],
                arbitrary_json: true,
                event_prefix: "bar".to_string(),
            }),
            get_resp.integration
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 2,
                result: vec![
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::Ifttt.into(),
                    },
                    api::IntegrationListItem {
                        kind: api::IntegrationKind::MqttGlobal.into(),
                    }
                ],
            },
            list_resp
        );

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteIftttIntegrationRequest {
                application_id: app.id.to_string(),
            },
        );
        let _ = service.delete_ifttt_integration(del_req).await.unwrap();

        // list
        let list_req = get_request(
            &u.id,
            api::ListIntegrationsRequest {
                application_id: app.id.to_string(),
            },
        );
        let list_resp = service.list_integrations(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(
            &api::ListIntegrationsResponse {
                total_count: 1,
                result: vec![api::IntegrationListItem {
                    kind: api::IntegrationKind::MqttGlobal.into(),
                },],
            },
            list_resp
        );
    }
}
