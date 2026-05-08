-- Test data for the public form routes.
--
-- This script assumes `server/migrations/001_setup.sql` and `server/migrations/002_form.sql`
-- have already run. It inserts the two forms currently hardcoded in
-- `server/src/http/public/form.rs`, plus share tokens that map to the existing public test
-- URLs:
--
--   /api/v1/f/550e8400-e29b-41d4-a716-446655440000
--   /api/v1/f/6f9619ff-8b86-4d01-a42d-7d0c3d8f9e10
--
-- `token_hash` values are SHA-256 hex digests of the raw UUID token strings above.

insert into form.form
    (form_id, version, title, description_markdown, active, form_section, created_at, created_by)
values
(
    '11111111-1111-1111-1111-111111111111',
    1,
    'Plasma Work Order Execution - Customer Confirmation',
    'This Request For Information document describes how the MES system handles plasma cutting work order execution. Please review each section carefully and confirm whether the described behavior matches your operation.',
    true,
    $json$
    [
      {
        "section_id": "plasma-execution-confirmations",
        "number": 1,
        "title": "Workflow confirmation items",
        "description_markdown": null,
        "questions": [
          {
            "question_id": "parts-grouped-for-cutting",
            "number": 1,
            "title": "How Parts Are Grouped for Cutting",
            "required": true,
            "kind": {
              "type": "validation",
              "description_markdown": "In your operation, the plasma table cuts a full sheet at a time. A single sheet can contain parts from multiple different jobs. The nesting software determines which parts are combined onto a sheet and assigns them a program name.\n\nBased on this, the system groups parts for cutting by program name. All parts that share a program name are treated as one unit of work at the plasma machine. This is called a cutting work order.\n\n### What this means in practice\n\n- One cutting work order = one program = one sheet of material\n- A single sheet can contain parts from multiple jobs and the system handles this correctly\n- The operator selects a program, not a job or a part\n- Good count and scrap count at end of run are recorded at the program level, not the individual part level",
              "confirm_prompt": "Parts from different jobs can appear on the same sheet, and the program name is the correct way to identify a cutting work order.",
              "summary_item": "Parts grouped by program name - one run per sheet"
            }
          },
          {
            "question_id": "cut-execution",
            "number": 2,
            "title": "Cut Execution",
            "required": true,
            "kind": {
              "type": "validation",
              "description_markdown": "Metal sheets are staged based on Fit Date at the Plasma Cutter Stations. A sheet is loaded onto the cutter table.\n\nThe Operator measures the sheet and enters the width and length in the MES.\n\nThe Operator then searches the associated Job Number / Program Number based on the dimensions from a dropdown list. The dropdown is pre-filtered for one (1) week's work orders based on fit date.\n\nThe Operator selects the appropriate line item that has the required Job Number / Program Number / Dimensions.\n\nThe available Lot Numbers are loaded into a dropdown list, from which the operator can match the Lot Number found written on the metal sheet to be cut.\n\nThe cutting process can now commence.",
              "confirm_prompt": "This cut execution process matches your current workflow.",
              "summary_item": "Cut execution by measured dimensions, job/program selection, and lot matching"
            }
          },
          {
            "question_id": "set-next-run",
            "number": 3,
            "title": "Set Next Run",
            "required": true,
            "kind": {
              "type": "validation",
              "description_markdown": "After the operator confirms the lot number, the program is queued as the next run on the machine. This is called Set Next Run.\n\n### What happens when Set Next Run is confirmed\n\n- The program is assigned to the plasma machine\n- The confirmed lot number is recorded against all parts in the program\n- The program moves to the top of the schedule and is marked as next\n- The operator can now see it on the production tab as the next planned order\n- No other program can be set as next on the same machine until this one is started or canceled\n\n> Note: Set Next Run is a commitment. The lot number is recorded at this point and is tied to the run. If the operator selected the wrong lot, a supervisor will need to intervene before the run is started.",
              "confirm_prompt": "Queuing a program as next and locking in the lot number at that point matches your workflow.",
              "summary_item": "Set Next Run locks in the lot number and queues the program"
            }
          },
          {
            "question_id": "starting-the-run",
            "number": 4,
            "title": "Starting the Run",
            "required": true,
            "kind": {
              "type": "validation",
              "description_markdown": "Once a program is set as next, the operator starts the run from the production tab on the dashboard.\n\n### What happens when Start Run is pressed\n\n- The run status changes from next to running\n- The actual start time is recorded\n- The production tab updates to show the current program, the part count, and a progress indicator\n- All parts in the program move to at plasma status and are no longer available to be queued on another machine\n\n> Note: The Start Run button is only enabled when there is a program queued as next. If no program has been set as next, the button will be disabled.",
              "confirm_prompt": "The operator starts the run from the production tab after setting the next run on the schedule tab, and this two-step process matches how your operators work.",
              "summary_item": "Start Run moves parts to at-plasma status and begins timing"
            }
          },
          {
            "question_id": "ending-the-run",
            "number": 5,
            "title": "Ending the Run",
            "required": true,
            "kind": {
              "type": "validation",
              "description_markdown": "When the plasma machine finishes cutting the sheet, the operator ends the run from the production tab.\n\n### What the operator enters at end of run\n\n- Good count: number of parts that came off the machine in acceptable condition\n- Scrap count: number of parts that were scrapped\n- When End Run is confirmed, the run is closed and the actual end time is recorded\n- Good count and scrap count are saved against the run record\n- All parts in the program move to plasma outfeed status and are physically done at the plasma table and ready to move to the next stage\n- The program is removed from the schedule screen\n- The production tab clears, ready for the next program\n\n> Note: Good count and scrap count are entered at the program level, not per part. If individual part-level scrap tracking is required, that would need to be handled as a separate requirement.",
              "confirm_prompt": "End of run captures good count and scrap count at the program level, and this is sufficient for your reporting needs.",
              "summary_item": "End Run captures good and scrap count at program level"
            }
          }
        ]
      },
      {
        "section_id": "wip-movement-note",
        "number": 2,
        "title": "What Comes Next - WIP Movement",
        "description_markdown": "After a run is ended, parts are physically moved from the plasma outfeed area to staging bins before they go to the bender. The system will track which bin each part is placed in via the WIP board. The WIP board and bin movement flow will be covered in a separate confirmation document.",
        "questions": []
      }
    ]
    $json$::jsonb,
    '2026-05-06T00:00:00Z',
    'Elev8'
),
(
    '22222222-2222-2222-2222-222222222222',
    1,
    'Simple Test Form',
    'A small form used to exercise text, choice, and multi-choice responses.',
    true,
    $json$
    [
      {
        "section_id": "general",
        "number": 1,
        "title": "General questions",
        "description_markdown": null,
        "questions": [
          {
            "question_id": "project-notes",
            "number": 1,
            "title": "Project Notes",
            "required": true,
            "kind": {
              "type": "text",
              "description_markdown": "Enter a short note for this test submission.",
              "placeholder": "Type notes here",
              "multiline": true,
              "max_length": 500
            }
          },
          {
            "question_id": "preferred-contact",
            "number": 2,
            "title": "Preferred Contact",
            "required": true,
            "kind": {
              "type": "choice",
              "description_markdown": "Choose one contact method.",
              "options": [
                { "question_option_id": "email", "label": "Email", "description": null },
                { "question_option_id": "phone", "label": "Phone", "description": null },
                { "question_option_id": "meeting", "label": "Meeting", "description": null }
              ],
              "allow_comment": true
            }
          },
          {
            "question_id": "requested-features",
            "number": 3,
            "title": "Requested Features",
            "required": false,
            "kind": {
              "type": "multi_choice",
              "description_markdown": "Select any features that apply.",
              "options": [
                { "question_option_id": "public_submission", "label": "Public submission", "description": null },
                { "question_option_id": "public_viewer", "label": "Public viewer", "description": null },
                { "question_option_id": "admin_builder", "label": "Admin builder", "description": null }
              ],
              "min_selected": null,
              "max_selected": null,
              "allow_comment": true
            }
          }
        ]
      }
    ]
    $json$::jsonb,
    '2026-05-06T00:00:00Z',
    'Elev8'
)
on conflict (form_id, version) do update
set
    title = excluded.title,
    description_markdown = excluded.description_markdown,
    active = excluded.active,
    form_section = excluded.form_section,
    updated_by = 'test seed';

insert into form.share_token
    (share_token_id, token_hash, token_prefix, form_id, form_version, active, expires_at, notes, created_at, created_by)
values
(
    '33333333-3333-3333-3333-333333333333',
    'a3a9e1ed9732cab28868127be00f1ce921acaefdd5c3b23a6e9e0072bd9c1a34',
    '550e8400',
    '11111111-1111-1111-1111-111111111111',
    1,
    true,
    '2027-05-06T00:00:00Z',
    'Public test token for the plasma RFI form.',
    '2026-05-06T00:00:00Z',
    'Elev8'
),
(
    '44444444-4444-4444-4444-444444444444',
    '5270b2833101b872383ed1e84084df9202724715c57649e81532f15768892a96',
    '6f9619ff',
    '22222222-2222-2222-2222-222222222222',
    1,
    true,
    '2027-05-06T00:00:00Z',
    'Public test token for the simple test form.',
    '2026-05-06T00:00:00Z',
    'Elev8'
)
on conflict (token_hash) do update
set
    active = excluded.active,
    expires_at = excluded.expires_at,
    used_at = null,
    notes = excluded.notes,
    updated_by = 'test seed';

-- Seed-only share tokens used to satisfy the completed_form audit relationship.
-- These are already consumed and are not the public form-fill test tokens above.
insert into form.share_token
    (
        share_token_id,
        token_hash,
        token_prefix,
        form_id,
        form_version,
        active,
        expires_at,
        used_at,
        notes,
        created_at,
        created_by
    )
values
(
    '77777777-7777-7777-7777-777777777777',
    'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
    'seedplsm',
    '11111111-1111-1111-1111-111111111111',
    1,
    false,
    '2027-05-06T00:00:00Z',
    '2026-05-06T15:45:00Z',
    'Consumed seed token for the plasma completed-form viewer example.',
    '2026-05-06T00:00:00Z',
    'Elev8'
),
(
    '88888888-8888-8888-8888-888888888888',
    'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb',
    'seedsmpl',
    '22222222-2222-2222-2222-222222222222',
    1,
    false,
    '2027-05-06T00:00:00Z',
    '2026-05-06T16:02:00Z',
    'Consumed seed token for the simple completed-form viewer example.',
    '2026-05-06T00:00:00Z',
    'Elev8'
)
on conflict (token_hash) do update
set
    active = excluded.active,
    expires_at = excluded.expires_at,
    used_at = excluded.used_at,
    notes = excluded.notes,
    updated_by = 'test seed';

insert into form.completed_form
    (
        completed_form_id,
        form_id,
        form_version,
        share_token_id,
        company_name,
        signer_name,
        signer_title,
        submitted_at
    )
values
(
    '55555555-5555-5555-5555-555555555555',
    '11111111-1111-1111-1111-111111111111',
    1,
    '77777777-7777-7777-7777-777777777777',
    'Example Fabrication Co.',
    'Jordan Smith',
    'Operations Manager',
    '2026-05-06T15:45:00Z'
),
(
    '66666666-6666-6666-6666-666666666666',
    '22222222-2222-2222-2222-222222222222',
    1,
    '88888888-8888-8888-8888-888888888888',
    'Example Customer LLC',
    'Taylor Lee',
    'Project Lead',
    '2026-05-06T16:00:00Z'
)
on conflict (completed_form_id) do update
set
    company_name = excluded.company_name,
    signer_name = excluded.signer_name,
    signer_title = excluded.signer_title,
    submitted_at = excluded.submitted_at;

delete from form.question_response
where completed_form_id in (
    '55555555-5555-5555-5555-555555555555',
    '66666666-6666-6666-6666-666666666666'
);

insert into form.question_response
    (completed_form_id, response, answered_at)
values
(
    '55555555-5555-5555-5555-555555555555',
    '{"question_id":"parts-grouped-for-cutting","response":{"type":"validation","status":"confirmed","comment":null}}'::jsonb,
    '2026-05-06T15:45:00Z'
),
(
    '55555555-5555-5555-5555-555555555555',
    '{"question_id":"cut-execution","response":{"type":"validation","status":"confirmed","comment":null}}'::jsonb,
    '2026-05-06T15:45:00Z'
),
(
    '55555555-5555-5555-5555-555555555555',
    '{"question_id":"set-next-run","response":{"type":"validation","status":"not_correct","comment":"Supervisors can override the next run before it starts."}}'::jsonb,
    '2026-05-06T15:45:00Z'
),
(
    '55555555-5555-5555-5555-555555555555',
    '{"question_id":"starting-the-run","response":{"type":"validation","status":"confirmed","comment":null}}'::jsonb,
    '2026-05-06T15:45:00Z'
),
(
    '55555555-5555-5555-5555-555555555555',
    '{"question_id":"ending-the-run","response":{"type":"validation","status":"not_correct","comment":"We also track scrap by part number for some customers."}}'::jsonb,
    '2026-05-06T15:45:00Z'
),
(
    '66666666-6666-6666-6666-666666666666',
    '{"question_id":"project-notes","response":{"type":"text","value":"Please include the public viewer in the first test pass."}}'::jsonb,
    '2026-05-06T16:00:00Z'
),
(
    '66666666-6666-6666-6666-666666666666',
    '{"question_id":"preferred-contact","response":{"type":"choice","selected_option_id":"email","comment":"Send the first draft to the operations alias."}}'::jsonb,
    '2026-05-06T16:01:00Z'
),
(
    '66666666-6666-6666-6666-666666666666',
    '{"question_id":"requested-features","response":{"type":"multi_choice","selected_option_ids":["public_submission","public_viewer"],"comment":null}}'::jsonb,
    '2026-05-06T16:02:00Z'
);

insert into form.viewer_token
    (
        viewer_token_id,
        token_hash,
        token_prefix,
        completed_form_id,
        active,
        expires_at,
        created_at,
        created_by
    )
values
(
    '99999999-9999-9999-9999-999999999999',
    '6316e01c9e1d33dec091e5469b8fe3f63ae270f7471846f380d926c3454b4027',
    '7c9e6679',
    '55555555-5555-5555-5555-555555555555',
    true,
    '2027-05-06T00:00:00Z',
    '2026-05-06T00:00:00Z',
    'Elev8'
),
(
    'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa',
    'c80e4e22ecf8c676675fe9f158e2b10d2af8428db8dc2add88be3f5591ad067b',
    '9f8c7b6a',
    '66666666-6666-6666-6666-666666666666',
    true,
    '2027-05-06T00:00:00Z',
    '2026-05-06T00:00:00Z',
    'Elev8'
)
on conflict (token_hash) do update
set
    active = excluded.active,
    expires_at = excluded.expires_at,
    updated_by = 'test seed';
