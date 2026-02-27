# Town Hall
A platform that connects those in need with open source coders.
## Terminology

| Term           | Definition                                                                                                                              |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| **Villager**   | A non-technical user who submits requests                                                                                               |
| **Quest**      | A request for a program with clearly defined requirements                                                                               |
| **Adventurer** | A (vibe or non-vibe) coder who works on Quests                                                                                          |
| **Guild**      | A group of Adventurers working together, structured similarly to an organizational account on GitHub                                    |
| **Solution**   | A submission containing a demo of an Adventurer's work and a GitHub link                                                                |
| **Council**    | A dispute resolution body composed of eligible Adventurers, convened to arbitrate conflicts between Villagers and Adventurers           |
| **Claim**      | A non-binding indicator that an Adventurer intends to work on a Quest; signals intent to other Adventurers without granting exclusivity |
| **Approval**   | A Villager's formal acceptance of a Solution, which marks the Quest as complete                                                         |

## Flow

1. A Villager fills out a form for their Quest, which is then scrutinized by an AI to check for edge cases, strict requirements, etc. before it goes live.
2. Adventurers must claim the Quest before they can talk to the Villager. A claim is not a lock; it serves only as an indicator for other Adventurers to see whether they want to participate as well. An Adventurer can only hold one active claim at a time. Adventurers who claim and go silent can be reported.
3. Adventurers can post questions below asking for more details about the Quest.
4. Adventurers post **Solutions** and the Villager can give feedback.
5. If the Villager is satisfied with the work, they can **Approve** it, making the Quest complete.

## Features

### Compensation (WILL NOT BE PRESENT IN MVP)
Villagers can provide compensation for projects, increasing Adventurer incentive to do this. The program is still required to be open source regardless. Compensation will be locked in before it goes live on the platform, ensuring it is there. Villagers can cancel the Quest along with the compensation within a set amount of time after it is posted.

### Quests
Quests can be upvoted or downvoted. Requests with more upvotes get more visibility, and vice versa. Villagers can limit their post visibility to x level or up, but then they'll have a lower amount of Adventurers that can work on it. Villagers would also be able to verify themselves as formal organizations/companies, much like what the old Twitter verification system was like.

### Solutions
Solutions must contain a GitHub link, a build process, and the project itself presented in an accessible way. It could be a website or a binary (build through GitHub CI/CD. So a link to GitHub workflows.) The smaller, the better: zero-dependency, local-first utilities are encouraged. Why make it React + Next.js when it could be a single Python script? Adventurers can also suggest existing software as a solution; if approved, they still get a level boost, just less. Solutions can be upvoted or downvoted, just like Quests; if an Adventurer upvotes a Solution that got accepted, they also get a slight amount of experience.

### Adventurers
Adventurers must log in with GitHub OAuth (also GitLab or others maybe?). They will be given a score based on their account activity. New accounts and low-scored accounts will be rate limited or even prohibited from participating in certain actions. Adventurers can accept and complete Quests to level up, and low level users can only see simpler Quests. Each Adventurer has a panel showcasing their level and completed projects, and they can include quotes from satisfied Villagers.

### Attribution
Adventurers must attribute any other Adventurer's code or any open source project they've used within the website's Solution submission box. Adventurers can report accepted submissions if the code was used and it wasn't attributed either in the source code or on the platform, which will result in punishments. A malicious report will also receive punishments. An attributed Adventurer, even if their answer is not accepted, will receive experience.

### Inspectors
Allow developers to inspect each other's work. If a bug is discovered in a completed project, they can submit it as a GitHub issue then link it back to our website. Other Adventurers can upvote the issue to increase visibility. At a threshold, the Villager is notified; if they confirm the existence of the bug, they can re-open the project (along with optional extra compensation), allowing either the original Adventurer or other Adventurers to fix the code and present a fixed version.

### Dispute Resolution

When a conflict arises between a Villager and an Adventurer (e.g. a Villager going inactive or a Solution that appears to meet all stated requirements but is refused) a moderator can open a Council to arbitrate the outcome. For now, only moderators can initiate a Council.

#### The Council

The Council is composed of Adventurers who have accumulated a sufficient credit score. Members are assigned cases randomly and cannot choose which disputes they review. Each Council member receives a limited number of cases per day (e.g., 3). A member may **Abstain** from a case, but doing so consumes one of their daily slots and forfeits the XP that voting would have earned. There are no additional penalties for abstaining. All proceedings are fully anonymous while a dispute is active. Once a dispute closes, the complete record (the original report, all votes, all reasoning, and their authors) becomes permanently public and attached to each participant's profile.

#### Phase 1: Blind Voting

Council members cast their votes independently, with no visibility into anyone else's choices. The moderator presents resolution options suited to the specific dispute, which may include outcomes such as ruling in favor of the Adventurer, ruling in favor of the Villager, re-opening the Quest with revised requirements, a no-fault dismissal, or others as appropriate.

#### Phase 2: Deliberation

Once Phase 1 closes, Phase 2 opens. Vote tallies remain hidden, but Council members can now read the reasoning others have submitted. A member may reinforce their original vote with written reasoning, or they may change their vote, in which case they are required to submit written reasoning explaining why. Rather than restating an argument already made, a member can endorse another member's reasoning as the basis for their own position. All submitted reasoning, whether to reinforce or to explain a swap, can be upvoted or downvoted by other Council members, which affects the author's XP accordingly. The original author of endorsed reasoning receives XP attribution for each endorsement. A significant swing between Phase 1 and Phase 2 results is treated as a signal that important details were missed in the first round, and the moderator may take this into account.

#### Resolution

If the final vote exceeds a defined majority threshold, the result is binding. If the result falls too close to call, the moderator can either supply additional context and prompt further deliberation, or issue a final decision directly. Council members who fail to engage (e.g. excessive abstention or consistently voting without submitting reasoning) will face consequences to their credit score and Council eligibility. Council members who are able to convince others or who consistently align with the majority outcome will receive a minor reward.

#### The Meta-Council

If a moderator determines that a Council's outcome warrants review, they can open a **Meta-Council** to dispute it. The Meta-Council follows the same structure, phases, and rules as a standard Council, with participants drawn from a separate pool. Both standard Councils and Meta-Councils are moderator-initiated only.
