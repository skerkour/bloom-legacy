# Workflow

## Overview

As explained in [Structure](structure.md), Bloom is a distributed organization.
It means no in person meetings, no dicussion arround the coffee machine. It also means that some
colleague or contributors may not live on the same timezone as you, thus, **everything should be written**.
Whether it be documentation, research, thoughts or anything else,

## Project management

* Project Spec goes into [The Guide](https://gitlab.com/bloom42/the_guide)
    * Extremely detailed with mockups and sketches, so there's little scope for confusion
    * With versioning and revisions

* Issues are created from the Spec
    * Create fine-grained issues for tasks that would typically take not more than 1-2 days
    * All issues have clear labels and a milestone
    * Usually end up with a large no. of issues, so correct labels are critical
    * Due-dates are set where applicable
    * Spec discussion/triage also takes place via GitLab issues (labelled appropriately)

* Sprints are managed through GitLab's epics

* Branches are created for each issue
    * Follow the GitFlow branching model
    * Commits go to the dev branch and are eventually merged into master when releasing
    * Once merged in master, code should be deployed in staging
    * Then once QA passed, it should be deployed to production

* Issues are also created for both bugs and improvements
    * Bugs should be labeled with a red 'bug' label
    * Improvements should be labeled with a green 'improvement' label


<!-- * Project Spec goes into [The Guide](https://www.buggycoder.com/project-management-with-gitlab)
  * Extremely detailed with mockups and sketches, so there's little scope for confusion
  * With versioning and revisions
* Issues are created from the Spec
  * Create fine-grained issues for tasks that would typically take not more than 1-2 days
  * All issues have clear labels and a milestone
  * Usually end up with a large no. of issues, so correct labels are critical
  * Due-dates are set where applicable
  * Spec discussion/triage also takes place via GitLab issues (labelled appropriately)
* Issues are added to the Kanban Board "to-do" list and follow the dev cycle
  * If things get re-prioritized or deferred, due date is updated accordingly and issues are moved out
* Code: Milestone branch for current milestone is created
  * Follow the GitFlow branching model
  * Commits go to the dev branch and are eventually merged into master when releasing
* Code: Issues from the to-do list are picked up and worked on
  * Commits for the issue are PR'd and reviewed (PR to milestone-release branch)
  * Issues are closed with a comment and moved out of Kanban Board
  * If changes are required they are reopened and rescheduled back to to-do queue
 -->

## Resources

* https://www.atlassian.com/agile/project-management/program
* https://blog.scottnonnenberg.com/an-agile-organization/
* https://www.buggycoder.com/project-management-with-gitlab/
