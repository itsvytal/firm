#let template(
  customer: "",
  title: "Technical retainer proposal",
  introduction: content,
  scope_covered: content,
  scope_excluded: content,
  hourly_rate: content,
  weekly_hours: content,
  schedule: content,
  contact_person: content,
  monthly_total: content,
  payment_terms: content,
  duration: content,
  exceptions: none,
  valid_until: none,
  acceptance_email: "daniel@42futures.com",
) = [
  #set document(title: [#title], author: "42futures")
  #let footer = [#text(fill: rgb("#c566ff"), weight: 800)[42futures]#h(1fr)#context counter(page).display("1 of 1", both: true)]
  #set page(paper: "a4", footer: footer)
  #set text(font: "SF Pro Display", size: 14pt)
  #set heading(numbering: "1.")
  #set par(justify: true)

  #align(center)[
    #text(32pt, weight: 800)[#customer + 42]
    #v(-1.5em)
    #text(16pt)[#title]
    #v(0.5in)
  ]

  = Introduction
  #introduction

  The retainer provides #weekly_hours of dedicated technical support each week, #schedule, for hands-on implementation and problem-solving.

  = How it works
  #weekly_hours reserved exclusively for your technical work every week.

  - *Schedule:* #schedule (Danish local time).
  - *Sessions:* Via Google Meet.
  - *Work priorities* set by #contact_person morning of or day before.

  == Scope
  Defined scope keeps our engagement focused where I can help most.

  === Covered topics
  #scope_covered

  === Excluded topics
  #scope_excluded

  = Investment
  - *Rate:* #hourly_rate per hour.
  - *Monthly commitment:* #monthly_total.
  - *Payment terms:* #payment_terms

  = Terms
  Initial duration of #duration, automatically renewing unless either party cancels before month-end.

  #if exceptions != none [
    #exceptions
  ]

  = Next steps
  #if valid_until != none [
    This proposal is valid until #valid_until.
  ] else [
    This proposal is valid for two weeks from its issue date.
  ]

  To proceed, email #acceptance_email and we'll finalize the contract.
]
