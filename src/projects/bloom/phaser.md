# Phaser

1. [Overview](#overview)
2. [Scenarios](#scenarios)
3. [Non goals](#non-goals)
4. [Models](#models)
5. [Views](#views)
6. [Open issues](#open-issues)

-------------------


## Overview

### CLI

--profile: A .sane file containing the scanner's profile. Default to 'network'
--targets: a file containing all targets, newline separated
--format: the output logging format [text, json]
--concurrency: max parallel targets to scan
--output: output folder where to store scan results
--debug: set Set logging level to debug
--assets: the assets folder (wordlists…)


les targets sont checkees a priori


### Worker (SaaS)

--assets: the assets folder (wordlists…)
--data: data directory where to store temporary scan data


support proxy


comment le plugger et faire en sorte qu'il detecte de maniere autonatique une infra (aws, gcp, azure) ?


un scanner de securite,
on peut ajouter des scans (targets ?):
- un nom
- type (application / network)
- une ou plusieur cibles (ip/domain if type == network, url if type == applicaiton)
- des cibles a exclure (memes regles que ci dessus, mais blob)
- authentification ? (application)
- user agent (application)
- max QPS
- schedule

le scan s'ajoute a la liste
il faut ensuite pouvoir le lancer a un instant T (si il n'est pas deja en cours)
consulter les resultats (vulnerabiltes, weaknesses)


pour l'api:
scans
reports

pour le scanner:
scan

targets
scans
reports
profiles
configs
runs
sessions


un demande un scan
il est queued,
il commence
il termine
le resultat est envoye a l'API
l'api le process et en fait un report

comment versionner les scan results ?
et donc la creation du report


On creer un dossier sur s3 (scans/id)
Le scanner y upload la data
Et report.json / scan.json / result.json
A la fin il envoie un message, l'api va chercher les donnees et genere le report

Ca pourrait permettre de generer des reports html en cli

on utilise des url relatives pour les donnees par rappirt au report ?

Exemple de donnees:
Crawled urls / port
Body des pages html
Screenshot



Report: ce sont les resultats d'un scan processés et lisibles par des humains (enrichis?)
Scan: Soit une alliance profile + targets, soit l'instance d'un scan,



report:
finding: data field ? details ?
findings in target ?

findings: ce sont des aggregat separes ? comment linker les donnees statiques ? (description, rist, name...)

## Scenarios


### 1

Sylvain wants to scan his website for vulnerabilities. he adds his website address to the scanner, run a scan and then consult results.


## Models

### Scan (run)
target: String

progress

events:
created
queued
started
completed
failed ?
deleted

status:
- queued
- scanning
- pending


### Report (result)
"created":
"started":
stopped":
"high_level_findings": 4,
"medium_level_findings": 7,
"low_level_findings": 11,
"information_findings": 18,
"cvss": {
      "type": "number",
      "description": "the overall CVSS score of the report"
    },
crawled_url_count?
findings: []
errors: [],
status

### Finding (issue)



## Views

### Scans
on liste les scans: name, targets, profile, last run, findings count, action (run, delete)
on peut crer un nouveau scan


### Profiles
on affiche seulement les 2 profiles network + application avec des explication pour chaque

### Reports
route
/{scan_id}/reports -> last report ?

voir detectify ?
en haut le graph dans le temps (avec selection de la periode ?)
en dessous les infos a propos du scan (findings by level, timing....) (title: report overview)

en dessous les findings (title: findings)


## Open issues
ets-ce vraiment utilse d'avoir un report.Status ?
- generer
- tracking over time
- scan history (au top level)
- personalised profiles
- est-ce qu'on utilise des profiles pour les settings ?



--------------------

https://developer.detectify.com/#scan-reports
https://cloud.google.com/security-scanner/docs/reference/rest/v1beta/projects.scanConfigs.scanRuns#ScanRun.ExecutionState


# C’este la responsabilité de worker ou de scanner d’envoyer a s3 ?
(Enlever scanner)

-> worker

Soit une fois le scan fais on envoie tous les fichiers (walk…) soit on les envoie 1 par 1 durant le scan.


http/apifuzz -> on donne un schema openapi -> et ca fuzz l’api

http/graphql/fuzz -> on donne un schema graphie, et ca fuzz l’api

# Run

1. Pull latest Docker image
```sh
$ docker pull registry.gitlab.com/bloom42/phaser:latest
```

2. Get the latest `phaser.template.sane` file and move it to `phaser.sane`
```sh
$ wget https://gitlab.com/bloom42/phaser/raw/dev/phaser.template.sane?inline=false
$ mv phaser.template.sane phaser.sane
```

3. Edit `phaser.sane` with correct values

4. Launch Docker container
```sh
$ docker run -d -v `pwd`/phaser.sane:/phaser/phaser.sane:ro registry.gitlab.com/bloom42/phaser:latest
```


# Build

```sh
$ make
```

# Development

```sh
$ make dev_docker_build
$ make dev_docker_run
```
