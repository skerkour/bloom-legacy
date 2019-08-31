# Workflow


* Product Spec goes into the Wiki
    * Extremely detailed with mockups and sketches, so there's little scope for confusion
    * With versioning and revisions
* Issues are created from the Spec
    * Create fine-grained issues
        * For tasks that would typically take not more than 1-2 days
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



Deployment flow:
each push -> tests on CI/CD
push new version on master -> deploy on staging
manual production deploy


* https://www.atlassian.com/agile/project-management/program
* https://blog.scottnonnenberg.com/an-agile-organization/
