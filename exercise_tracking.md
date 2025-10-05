# Goal

Get a first version of exercise tracking working.

Support only scales and songs for now.

There must be a general distinction between Exercises that can just be selected like scales and exercises that have to be configured like songs.

Features:

- Exercises selection
  - Make it possible to start a scale exercise
  - Make it possible to open a practice session of a song
  - Make it possible to create songs to practice

- Songs
  - There are three different "layers"
    - Template for creating a song exercise
    - Created song to practice
    - Practice Session of the song
      - At this stage we can save with which bpm the song was saved

- When the exercise is started, do not make it possible to change the exercise
  - Create a dialog
    - when
      - the user wants to change something
        - alternatively disable the dropdowns for changing the exercise when the timer is started
      - when trying to leave the page
    - Dialog options: discard exercise, save and change and cancel

- BPM can be changed on the fly and we just save the last used BPM for the exercise, at least for this first version

## Current TODOs

- preview in start scale should contain notes
- actually implement saving a song
- Fix inline editability of exercise properties
  - ❌ Show dialog when session is in progress and trying to change a property
    - ✅ or maybe disable the buttons when the session is started
  - Fix warnings on exercise page
- Fix dark mode
  - Weird slider toggle initial state problem
  - In Practice Session

# Next Steps

Implement tracking

- when pressing stop save the data of the current session for tracking
- Implement viewing stats
  - simple first iteration, maybe just number of pracice session plotted over time

# Ideas

- Inline history of tracking in exercisers
  - would be interesting for songs and scales and in general for all types
