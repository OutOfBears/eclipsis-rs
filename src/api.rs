use crate::{
    apis::configuration::Configuration,
    apis::user_api::*,
    apis::Error,
    apis::{client_api::*, match_api::*},
    models::*,
};

macro_rules! call_api {
    ($self:ident, $api_call:ident, $enum:ident, $params:expr) => {{
        let resp = $api_call(&$self.configuration, $params).await?;
        let content = resp.entity.ok_or(ApiError::UnableToParseResponse)?;

        match content {
            $enum::Status200(data) => {
                return if data.success.unwrap_or(false) {
                    Ok(data.value.expect("value expected"))
                } else {
                    Err(ApiError::NotSuccessful)
                }
            }
            _ => return Err(ApiError::InvalidResponse),
        }
    }};

    ($self:ident, $api_call:ident, $enum:ident) => {{
        let resp = $api_call(&$self.configuration).await?;
        let content = resp.entity.ok_or(ApiError::UnableToParseResponse)?;

        match content {
            $enum::Status200(data) => {
                return if data.success.unwrap_or(false) {
                    Ok(data.value.expect("value expected"))
                } else {
                    Err(ApiError::NotSuccessful)
                }
            }
            _ => return Err(ApiError::InvalidResponse),
        }
    }};
}

macro_rules! get_api_key {
    ($self:ident) => {{
        $self
            .configuration
            .api_key
            .clone()
            .expect("api-key expected")
            .key
    }};
}

pub struct EclipseApi {
    configuration: Configuration,
}

#[derive(Debug)]
pub enum ApiError<T> {
    NotSuccessful,
    ResponseError(Error<T>),
    UnableToParseResponse,
    InvalidResponse,
}

type Result<T, E> = std::result::Result<T, ApiError<E>>;

impl EclipseApi {
    pub fn new(configuration: Configuration) -> EclipseApi {
        EclipseApi { configuration }
    }

    pub async fn delete_api_key(&self) -> Result<String, ClientDeleteError> {
        call_api!(
            self,
            client_delete,
            ClientDeleteSuccess,
            ClientDeleteParams {
                api_key: Some(get_api_key!(self)),
            }
        )
    }

    pub async fn info_api_key(&self) -> Result<Box<ApiKeyData>, ClientInfoError> {
        call_api!(self, client_info, ClientInfoSuccess)
    }

    pub async fn client_login(&self) -> Result<Vec<String>, ClientLoginError> {
        call_api!(
            self,
            client_login,
            ClientLoginSuccess,
            ClientLoginParams {
                api_key: Some(get_api_key!(self)),
            }
        )
    }

    pub async fn client_logout(&self) -> Result<String, ClientLogoutError> {
        call_api!(self, client_logout, ClientLogoutSuccess)
    }

    pub async fn get_match_data(
        &self,
        match_id: String,
    ) -> Result<Box<crate::models::Match>, GetMatchError> {
        call_api!(
            self,
            get_match,
            GetMatchSuccess,
            GetMatchParams { match_id }
        )
    }

    pub async fn get_user_rating_delta(&self, user_id: u64) -> Result<Vec<i32>, GetUserDeltaError> {
        call_api!(
            self,
            get_user_delta,
            GetUserDeltaSuccess,
            GetUserDeltaParams {
                user_id: user_id as i64,
            }
        )
    }

    pub async fn get_user_matches(&self, user_id: u64) -> Result<Vec<String>, GetUserMatchesError> {
        call_api!(
            self,
            get_user_matches,
            GetUserMatchesSuccess,
            GetUserMatchesParams {
                user_id: user_id as i64,
            }
        )
    }

    pub async fn get_user_overview(
        &self,
        user_id: u64,
    ) -> Result<Vec<Vec<i64>>, GetUserOverviewError> {
        call_api!(
            self,
            get_user_overview,
            GetUserOverviewSuccess,
            GetUserOverviewParams {
                user_id: user_id as i64,
            }
        )
    }

    pub async fn get_user_playtime(&self, user_id: u64) -> Result<i64, GetUserPlaytimeError> {
        call_api!(
            self,
            get_user_playtime,
            GetUserPlaytimeSuccess,
            GetUserPlaytimeParams {
                user_id: user_id as i64,
            }
        )
    }

    pub async fn get_user_rating(&self, user_id: u64) -> Result<i64, GetUserRatingError> {
        call_api!(
            self,
            get_user_rating,
            GetUserRatingSuccess,
            GetUserRatingParams {
                user_id: user_id as i64,
            }
        )
    }

    pub async fn get_user_status(
        &self,
        user_id: Vec<u64>,
    ) -> Result<Vec<Vec<i64>>, GetUserStatusError> {
        call_api!(
            self,
            get_user_status,
            GetUserStatusSuccess,
            GetUserStatusParams {
                request_body: user_id.into_iter().map(|id| id as i64).collect(),
            }
        )
    }

    pub async fn get_user_teammates(
        &self,
        user_id: u64,
    ) -> Result<Vec<Vec<i64>>, GetUserTeammatesError> {
        call_api!(
            self,
            get_user_teammates,
            GetUserTeammatesSuccess,
            GetUserTeammatesParams {
                user_id: user_id as i64,
            }
        )
    }
}

impl<T> From<Error<T>> for ApiError<T> {
    fn from(error: Error<T>) -> Self {
        ApiError::ResponseError(error)
    }
}
