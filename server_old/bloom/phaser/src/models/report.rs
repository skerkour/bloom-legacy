use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::findings::Finding;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Report {
    V1(ReportV1),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReportV1 {
    pub id: uuid::Uuid,
    pub scan_id: uuid::Uuid,
    pub config: ConfigV1,
    pub targets: Vec<Target>,
    pub phaser_version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConfigV1 {
    pub data_folder: String,
    pub assets_folder: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Target {
    pub host: String,
    pub kind: TargetKind,
    pub ip_version: Option<IpVersion>,
    pub findings: HashMap<ModuleName, Finding>,
    // pub errors: Vec<TargetError>,
    pub subdomains: Vec<Target>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TargetKind {
    Domain,
    Ip,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum IpVersion {
    V4,
    V6,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum ModuleName {
    #[serde(rename = "http/directory-listing")]
    HttpDirectoryListing,
    #[serde(rename = "http/ds-store")]
    HttpDsStore,
    #[serde(rename = "http/dotenv")]
    HttpDotenv,
    #[serde(rename = "ssltls/robot")]
    SsltlsRobot,
    #[serde(rename = "ssltls/cve-2014-0224")]
    SsltlsCve20140224,
    #[serde(rename = "ssltls/cve-2014-0160")]
    SsltlsCve20140160,
    #[serde(rename = "postgresql/unauthenticated-access")]
    PostgresqlUnauthenticatedAccess,
    #[serde(rename = "ports")]
    Ports,
    #[serde(rename = "mysql/unauthenticated-access")]
    MysqlUnauthenticatedAccess,
    #[serde(rename = "domain/axfr")]
    DomainAxfr,
    #[serde(rename = "domain/cname")]
    DomainCname,
    #[serde(rename = "domain/dmarc")]
    DomainDmarc,
    #[serde(rename = "domain/spf")]
    DomainSpf,
    #[serde(rename = "domain/subdomains")]
    DomainSubdomains,
    #[serde(rename = "domain/takeover")]
    DomainTakeover,
    #[serde(rename = "domain/whois")]
    DomainWhois,
    #[serde(rename = "http/atlassian/cve-2017-9506")]
    HttpAtlassianCve20179506,
    #[serde(rename = "http/cadvisor/unauthenticated-access")]
    HttpCadvisorUnauthenticatedAccess,
    #[serde(rename = "http/consul/unauthenticated-access")]
    HttpConsulUnauthenticatedAccess,
    #[serde(rename = "http/drupal/cve-2018-7600")]
    HttpDrupalCve20187600,
    #[serde(rename = "http/elasticsearch/unauthenticated-access")]
    HttpElasticsearchUnauthenticatedAccess,
    #[serde(rename = "http/etcd/unauthenticated-access")]
    HttpEtcdUnauthenticatedAccess,
    #[serde(rename = "http/yaheiphp/unauthenticated-access")]
    HttpYaheiphpUnauthenticatedAccess,
    #[serde(rename = "http/traefik/dashboard-unauthenticated-access")]
    HttpTraefikDashboardUnauthenticatedAccess,
    #[serde(rename = "http/prometheus/dashboard-unauthenticated-access")]
    HttpPrometheusDashboardUnauthenticatedAccess,
    #[serde(rename = "http/kibana/unauthenticated-access")]
    HttpKibanaUnauthenticatedAccess,
    #[serde(rename = "http/jetty/cve-2015-2080")]
    HttpJettyCve20152080,
    #[serde(rename = "http/gitlab/open-registration")]
    HttpGitlabOpenRegistration,
    #[serde(rename = "http/git/config-disclosure")]
    HttpGitConfigDisclosure,
    #[serde(rename = "http/git/directory-disclosure")]
    HttpGitDirectoryDisclosure,
    #[serde(rename = "http/git/head-disclosure")]
    HttpGitheadDisclosure,
}
