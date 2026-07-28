
# enneagram

The enneagram is often described as a personality-model. In reality, a wide range of mappings can be applied to it.

To describe this program as a personality-type questionnaire would be a gross simplification. It allows an enneatype to be confidently identified by answering three multiple-choice questions, that is if the fundamental presuppositions of the underlying model are sound, according to the enneagrams mathematical structure.

The enneagram is a nine-sided graph, with interconnections in the shape of a lotus flower. The interconnections form two loops, one triangular loop at points 3-6-9 and one loop at points 1-4-2-8-5-7.

The sides, or points of the graph are called enneatypes, or edges.

An inherent property of the enneagram is its triads, which are discussed in further detail below. Each enneatype belongs to a combination of four triads. These triads are combinatorically linked, correlated, in the sense that the full space of possibilities is fully defined by two of them. This means if two of the triads are defined, an enneatype can be determined, and the two other corresponding triads as well.

Whether this provides a true insight into the human psyché is not for me to decide. Ultimately, this program was an experiment to test the validity of the model. You can try it, and decide for yourself.

I want to be perfectly clear that, according to the defining literature of this model, its origins are said to be occult. There is a strangely mystical aspect that becomes apparent with use of the program, that to me seems uncanny. With the risk of appearing superstitious, i recommend users of this software to be aware of the occult origins of the underlying system that this software bases itself on. From experience, while captivating and bizarre, an overreliance of this software for self-help purposes may lead to madness or distress, and should be used with caution.

In a later addition, I added configurability to the enneagram software. This means, you can create your own user-defined enneagram in a `.yaml`, and decide for yourself every labeling and question within the program, fitting it to your own model. It does not need to strictly pertain to personality.

## Installation

The enneagram command-line interface application supports UNIX-based systems primarily (Linux, MacOS etc.). I'm uncertain whether or not it works on Windows. If you're curious about trying it, you can install it through the `pacman` package-manager from the AUR or from the `cargo` package manager, or alternatively clone the git repo and build it from there.

If you're running an Arch-based Linux distro, you can install it from the AUR:

```bash
yay -S enneagram
```

If not, you should be able to install it with `cargo`:

```bash
cargo install enneagram
```

If you're not sure how to install `cargo`, you can follow [this guide](https://doc.rust-lang.org/cargo/getting-started/installation.html).

The program should be safe to run on your computer, but keep in mind its origins are occult.

## Basic features

### Graphical artwork

If your terminal supports graphics, (like for instance, [`kitty`](https://wiki.archlinux.org/title/Kitty)), then it should be able to display the graphical artwork. You can view the graphical representation of the enneagram by running the following command:

```bash
enneagram -a
```

### Performing the evaluation

Your enneatype can be evaluated in three steps, by entering the following command:

```bash
enneagram
```

You will then be prompted with three questions. Answer carefully. You can try to give it your own answers to evaluate yourself, or answer from the viewpoint of someone else to evaluate them. Enter the number corresponding to your answer for each question.

The first question will ask you to provide a framing, or domain for the rest of your questioning. Pick whatever you like.

You will then be asked to select the traid-pair combination in the two remaining questions. Try to be as honest with yourself as possible. The alternatives are supposed to be equally weighted in desireability, so that no objective preferred answer exists. They all come with a caveat.

You will then be met with an evaluation of your enneatype, and a follow-up question. The follow-up question, or pivot, allows you to navigate the lines of freedom within the enneagram by confronting a dillemma. The second answer of the dillemma leaves you remaining where you are. You can keep answering the follow-up questions until you land on a steady-state answer.

Here's an example:

```
Q: please select a domain
A: desire-machine -> body without organs

Q: I am my thoughts, my fear hurts me and I need control
A: you believe you can change it and you tell yourself that everything is fine

Enneagram 7 Disorganization

792 Positive/"everything is fine"
567 Head/"I am my thoughts, my fear hurts me"
147 Frustration/"I need control"
378 Assertive/"I can change it"

Q: how will you handle your fear?
A: i will follow my gut, take responsibility, and tolerate it.

Enneagram 1 Recovery

135 Competent/"I take responsibility"
891 Gut/"I am my urges, my concience hurts me"
147 Frustration/"I need control"
612 Compliant/"I can tolerate it"

Q: how will you manage your frustration?
A: i will follow my heart, blame others, and avoid it.

Enneagram 4 Rejection

468 Reactive/"it's their fault"
234 Heart/"I am my emotions, my feelings hurt me"
147 Frustration/"I need control"
945 Withdrawn/"I can avoid it"

Q: how will you deal with your longing?
A: i will follow my gut, take responsibility, and tolerate it.

Enneagram 1 Recovery

135 Competent/"I take responsibility"
891 Gut/"I am my urges, my concience hurts me"
147 Frustration/"I need control"
612 Compliant/"I can tolerate it"

Q: how will you manage your frustration?
A: i will follow my gut, take responsibility, and tolerate it.
```

The initial enneatype here was *enneagram 7: disorganisation*. It was assigned to us, because of the first assertion of 'I am my thoughts, my fear hurts me and i need control'. We are met with fear, and the program supposes that we try to change it by way of denial. We choose instead to tolerate it through resilience and duty, and move to *enneagram 1: recovery*. Now we are met with frustration. To satisfy our frustration, we decide to reject responsibility and follow our heart. We are met with longing and loneliness. We choose to 'get a grip' and tolerate it, by taking responsibility, and as such, return to *enneagram 1: recovery*. Now we are faced with frustration, once again, but this time, after reviewing our options, we decide to follow our gut and take responsiblity. The End. Makes sense?

### Inspecting enneatypes

You can view the combinations of different enneatypes and how they relate by appending them as numeric arguments. Example:

```bash
enneagram 1 23
```

If used with the graphical artwork, the selected enneatypes will be highlighted:

```bash
enneagram -a 1 23
```

### Disabling pivot

You can perform the evaluation without pivot-questions by using the `-!p` flag (the exclamation-mark means *not*):

```bash
enneagram -!p
```

### Using a custom configuration

The enneagram CLI supports custom configurations. You can apply it using the `-c` flag, followed by the name of the config or a path to it.

```bash
enneagram -c unigram
enneagram -c christ
enneagram -c schizogram
enneagram -c ./my_custom_enneagram.yaml
```

The configuration is in the form of a `.yaml`-file. See the directory `~/.config/enneagram` for examples. The location may differ on other operating systems.

The configuration can specify every phrase and naming of enneatypes, triads, questions and answers, as well as the coloration of the graphic artwork.

## Rationale

The following is the initial flow-of-consciousness-style formulation of the program. It might seem incoherent, but these were my notes when i first thought of the idea on how the program should function. Take from it what you can.

### Introduction to the enneagram

Although you can often find it in fiction, i don't think i've ever met a real person in my life who is persistantly found at one of the arms of the nonagon at all times. We all move around in it from day to day or hour to hour. I would imagine that you'd agree to some extent. You mentioned several times how it can be repurposed as a lens to view momentary motivations or strategies. The whole thing seems like an emergant combinatoric pattern that takes form when one takes personality and describe it as the composition of [internal suffering/frame] + [external means/action] + [internal means/fault] + [external suffering/need], and define the system as consisting of two free variables where each variable (dependent or independent) has three possible states.

Schopenhauer would say that all experience is derivative of suffering (right or not).

To be clear: all of this describes the mind, so it's all internal, but by "internal" i mean more internal than the internal.

We can take the axiom of suffering = thesis, means = anti-thesis, personality = synthesis, in Hegel's model of: thesis + anti-thesis = synthesis, but modified to separate the internal and the external, and it starts to look familiar.

The room of possibilities contains two free variables. So identifying two of the four triads is enough to narrow down a specific personality type within this system (accurate or not). So supposedly, knowing only two of these should be enough to predict the two other. I'm not sure if that's the case.

If not, we'd have 3^4 = 27 personality types instead of 9.

Humonculus of the self/internalization of self/frame of judgement/meta-objective/"Who am i?":
891 - Gut/"i am my urges, my concience hurts me"
567 - Head/"i am my thoughts, my fear hurts me"
234 - Heart/"i am my emotions, my feelings hurt me"

External strategy towards suffering:
378 - Assertive/"i can change it"
612 - Compliant/"i can tolerate it"
945 - Withdrawn/"i can avoid it"

Internal strategy for one's (meta-)suffering/"who to blame?":
792 - Positive/"Everything is fine"
135 - Competent/"I take responsibility"
468 - Reactive/"it's their fault"

Need/object of desire/"what hole do you have in your soul?"
369 - Attachment/"i need freedom"
147 - Frustration/"i need control"
258 - Rejection/"i need love"

There's a few things that are still a bit unclear to me. Are there combinations of the above that are not covered by the enneagram's 9 personality types? (could there in fact be 27 in total?)  What's the deal with the interconnections/lines? Do they describe structural or dynamic pathways or relations between the types?

### On causality

Enneagram triad pairs:
IN = External dissonance
    - Thesis: need
    - Anti-thesis: fault
EN = External conflict
    - Thesis: need
    - Anti-thesis: action
EI = Behaviour
    - Introverted: fault
    - Extroverted: action
HN = Suffering
    - Introverted: frame
    - Extroverted: need
HI = Internal conflict
    - Thesis: frame
    - Anti-thesis: fault
HE = Internal dissonance
    - Thesis: frame
    - Anti-thesis: action

Cause and effect:
HE->IN = Internal dissonance predicts external dissonance
HI->EN = Internal conflict predicts external conflict
HN->EI = Suffering predicts behaviour
EI->HN = Behaviour indicates suffering
EN->HI = External conflict indicates internal conflict
IN->HE = External dissonance indicates internal dissonance

1) Recovery / Gradient
H = I am my urges, my concience hurts me
E = I can tolerate it
I = I take responsibility
N = I need control

2) Association / Superego
H = I am my emotions, my feelings hurt me
E = I can tolerate it
I = Everything is fine
N = I need love

3) Repression / Ego
H = I am my emotions, my feelings hurt me
E = I can change it
I = I take responsibility
N = I need freedom

4) Rejection / Id
H = I am my emotions, my feelings hurt me
E = I can avoid it
I = It's their fault
N = I need control

5) Catatonia 
H = I am my thoughts, my fear hurts me
E = I can avoid it
I = I take responsibility
N = I need love

6) Paranoia / 
H = I am my thoughts, my fear hurts me
E = I can tolerate it
I = It's their fault
N = I need freedom

7) Disorganization /
H = I am my thoughts, my fear hurts me
E = I can change it
I = Everything is fine
N = I need control

8) Action /
H = I am my urges, my conscience hurts me
E = I can change it
I = It's their fault
N = I need love

9) Rest /
H = I am my urges, my conscience hurts me
E = I can avoid it
I = Everything is fine
N = I need freedom

### Enneagram, the computer program

I'm writing a computer program that's essentially like a magic 8 ball but with basis on the enneagram/schizogram.

The combinatorics of it all makes it possible to determine one's placement on the enneagram if a pair of triads are given. The two other triads are then dependent. Looking at all the combinations, i've come to find that this combinatoric dependence between the triads (two degrees of freedom) makes emergent sense. It's interesting how the supposed answers for each question (consisting of two triads) results in a mostly reasonable assumption about someone's inner and outer spiritual conflict. A pair of triads can be referred to as either: a cause, or an effect. The other cause, consisting of another pair, can be determined from any effect and vice versa. Both pairs are dependent variables. If you have defined one pair, the other is pre-determined. Each cause/effect consists of a pair of triads of the enneagram.

So by having a computer program ask three questions, you can, (surprisingly enough!) get a reasonably useful answer from it:
1. Which kind of inferrence will we make?
- Internal dissonance (humonculus + external strategy)
- internal conflict (humonculus + internal strategy)
- suffering (humonculus + need)
- behaviour (external and internal strategy)
- external conflict (external strategy + need)
- external dissonance (internal strategy + need)
2. Select first triad
3. Select second triad
Voilla: an answer will be produced! That's the idea at least.

The conflicts are congruant while the dissonances are incongruant. Often, two triads reside in the conscious and the other two reside in the subconscious.

### Interconnections

I've figured out by now that the lines between each state make perfect sense. They'd point to the other state you would get if you were to change one of the two known triads. I could allow the user/caster to make a choice after their placement has been determined, to change one triad and thereby move between the enneagram's edges.

### The zeroth enmeatype

What i dub the zeroth enneatype, due to its parallel counterpart with the zeroth numagram-type, is the precondition itself. The axiom, so to speak, from which all the other types emerge from. In this case, it was Schopenhauer's statement of all experience stemming from suffering (as far as i can remember, from reading 'On Pessimism').

Turns out this assumption may not always be correct. If incorrect, it has the unfortunate consequence of creating a desire to make said precondition correct.

In a way, the zeroth becomes the price which you pay for each use of the program, inflicted upon the subject which it is used on if not already present.

### On The Holy Mountain

I just watched Jodorowsky's The Holy Mountain. The enneagram seems to reappear a lot in that movie as a symbol. There are interesting connections between the characters' plot points across the movie and the enneagram. The movie mainly focuses on sin, and how to conquer it in order to climb The Holy Mountain.

### Experimental results of the schizogram

I tried to use this enneagram program for self-evaluation for a month or so, and only occationally using it on others to better understand their inner turmoil. Turns out, with the current setup, it only works given the axiom that emotional turmoil is present to begin with.

If used on other people, this has the unfortunate unintended side-effect that emotional turmoil is wished on others in order to be able to understand them through the enneagram. A truly mischevious consequence, whose ramifications i realized far too late for my own good.

Experiments are not conclusive, but so far, points towards the model being too simplistic, taking each archetype to the extreme, without accounding for neuances, and often being axiomatically wrong, which has grave consequenses.

I occationally used it on others, mostly with their awareness. It did function to make me more charismatic towards women, but it also have made me a more suspicious and malevolent towards people i previously considered friends, and once seen-through (the curtain being revealed), it made me appear as creepy, dishonest and manipulative. I suppose it's only prøven purpose, then, is to effectively scare people.

#### Effect on the self

It can be assumed, that, as if by wish-fulfillment, one becomes more as one described onself when making the enneagram, except only a simplistic charicature of the logical extremes of each archetype. This is an effective way to achieve a rapid mental breakdown.

Using this program made me selfish, lonely, destructive, highly-emotional (turns out it created disregulation, not regulation.) and unable to think in neuances.

Eventually, i lost the sense of self entirely, and only became what others saw in me.

This may have been affected by the fact that i was going through a breakup at the time, from a semi-imaginary relationship in my head with a woman who is by far more troubled than i am. She recommended me a book, which i will read in order to better figure out how to live with our disorder, which we share. Deleuze and Guattari's Anti-Oedipus was her recomendation to me. Turns out she was right all along, and i didn't listen, because i was stubborn. But that's no use, because i now see after finally getting to read said book that being in love with her affects me negatively, because i become whoever people want from me, and she wants me to leave her alone. If i have to be destroyed in order to leave her alone, then, in her mind; so be it. I cannot help her, and i need to stop deluding myself into thinking that i'm doing anything but making it worse for the both of us.

This might have tainted the test results of this experiment somewhat, but she was the one who gave me the idea to make this program, after all, so in a way, this software is a monument to the relationship we had.

#### Effect on others

This program made me untrustworthy, and hated by everyone around me. It caused great fear and distress to everyone in my vicinity.

The idea of some esoteric magical spell computer program used to influence others is inherantly creepy and unhinged, and no basis for any good relationship.

Due to the inherant design flaw of being based on the axiom of emotional suffering, clearly it creates a need for that precondition to be in place to begin with. One therefore becomes desiring of that very precondition in others, subconciously guiding the caster of the program to want that very precondition to occur, in order to effectively use their tool of desire-reaching.

Therefore, it made me subconsciously become distressfull, so that everyone who were to be studied using the enneagram were caused distress.

Clearly there is an urgent need for the use of multiple enneagrams. The one i made, which was created at a very strange time of my life, was not ideal and caused grief in others.

### The unigram

I've now implemented the unigram as a yaml config for the enneagram program using the fundamental presupposition i.e. the 0-th of "the fundamental root of experience is creativity", as opposed to the Schopenhauerian notion of "the fundamental root of experience is suffering" which i used for the schizogram. It's still currently under development, but it looks promising as a guide for creative pursuits. It's exciting to see how these different ngrams can be used to enter various modes of experience. They are an ancient form of magic. I'm looking forward to experimenting with it, and seeing the results. I have hope that it may be different than the madness-inducing effects of the schizogram. I'm sure it can be used for good!

### In conclusion

This program is a tool for introspection with occult origins. Its source code is available to view online, on [`github`](https://github.com/sigurd4/enneagram). You may review the code as you like.

## Acknowledgements

- Claudio Naranjo's lectures on the enneagram.
- [CyberYamu](https://www.youtube.com/@CyberYamu)'s videos on the enneagram.
