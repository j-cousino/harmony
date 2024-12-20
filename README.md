# harmony
Distributed industrial design/CAD data mangement

Allow a user to move data from a PLM server such as Teamcenter or Aras and work on it locally.

The system should provide the following:
  - Record modifications made while in the users possesion.
  - Merge modified data from the server and understand if there are any conflicting modificatoions.
  - Allow reference data, changes are not tracked but are reported when merging new representative data.
  - Allow filename changes locally, but tranform them back to origianl name when preparing to move back.

    This is to allow for systems that use filenaming to determine the items ID.
