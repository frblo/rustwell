// This template is directly based on the 'celluloid' typst template
// by Casey Dahlin. See https://typst.app/universe/package/celluloid for
// the original template. This template has been heavily modified and is
// not fit for use outside of Rustwell, we instead refer to using the
// fantastic celluloid template for writing screenplays in typst. Below
// is the original copyright notice from celluloid.

// Copyright (C) 2025 Casey Dahlin <sadmac@google.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//         http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#let line_spacing(blanks) = 0.65em + blanks * 1.23em
#let dialogue_counter = counter("dialogue")

#let screenplay(
  titlepage: false,
  title: none,
  credit: none,
  authors: none,
  source: none,
  draft_date: none,
  contact: none,
  doc,
) = {
  set page(
    margin: ( top: 1in, left: 1.5in, right: 1in, bottom: 0.5in),
    header-ascent: 0.5in,
  )
  set text(
    font: "Courier Prime",
    size: 12pt,
  )
  set par(spacing: line_spacing(1))

  show heading: h => {
    set text(size: 12pt, weight: "regular")
    set block(spacing: line_spacing(1))
    h
  }

  if titlepage {
    page({
      align(center, {
        upper(title)

        if credit != none {
        block(credit, above: 1in)
      }
        if authors != none {
        block(authors, spacing: 0.5in)
      }
        if source != none {
        block(source, spacing: 0.6in)
      }
        if draft_date != none {
        block(draft_date, spacing: 0.9in)
      }
      })

      if contact != none {
      align(bottom, box(align(left, contact)))
    }
      counter(page).update(0)
    }, margin: ( top: 2in, bottom: 1in ))
  }

  set page(
    header: context {
      if counter(page).get().at(0) > 0 {
        align(right, counter(page).display("1."))
      }
    },
  )

  doc
}

#let dialogue_raw(character, dialogue, paren: none, left_inset, left_inset_name, right_inset_name) = {
  context {
    set par(spacing: line_spacing(0))
    dialogue_counter.step()
    let dialogue_count = dialogue_counter.get().at(0)
    let dialogue_header_counter = counter("dialogue_header" + str(dialogue_count))
    let dialogue_footer_counter = counter("dialogue_footer" + str(dialogue_count))
    grid(
      grid.header(block(par([#upper(context {
        dialogue_header_counter.step()
        let paren = if dialogue_header_counter.get() != (0,) {
          "CONT’D"
        } else {
          paren
        }
        character
        if paren != none {
          [ (#paren)]
        }
      })]), inset: (left: left_inset))),
      block(dialogue, spacing: line_spacing(0)),
      grid.footer(block({
        dialogue_footer_counter.step()
        context {
          if dialogue_footer_counter.get() != dialogue_footer_counter.final() {
            [(MORE)]
          }
        }
      }, inset: (left: left_inset), spacing: line_spacing(0))),
      inset: (left: left_inset_name, right: right_inset_name),
      gutter: line_spacing(0),
    )
  }
}

#let dialogue(character, dialogue, paren: none) = dialogue_raw(character, dialogue, paren: paren, 1.5in, 1in, 1.5in)

#let dual_dialogue(character1, dialogue1, paren1: none, character2, dialogue2, paren2: none) = grid(
  columns: 2,
  dialogue_raw(character1, dialogue1, paren: paren1, 1in, 0.5in, 1in),
  dialogue_raw(character2, dialogue2, paren: paren2, 1in, 0.5in, 1in),
)

#let lyrics(cont) = {
  block(
    upper(cont),
    above: line_spacing(0),
    inset: (left: 1in, right: 1.5in),
  )
}

#let parenthetical(content) = {
  context{
    block(
      par([#content], hanging-indent: measure("(").width),
      inset: (left: 0.5in, right: 1in), spacing: line_spacing(0)
    )
  }
}

#let scene(cont, number: none) = {
  grid(
    columns: (0em, 1fr, 0em),
    place(dx: -3em, align(right, number)),
    heading(block(upper(cont), above: line_spacing(2), below: line_spacing(1))),
    place(dx: 1em, number),
  )
}

#let centered(cont) = {
  block({
    align(center, block(cont))
  }, breakable: false, width: 100%, below: line_spacing(2))
}

#let transition(name) = {
  align(right, block([#upper(name)], spacing: line_spacing(1), inset: (right: 2em)))
}

#let synopsis(cont) = {
  block(
    inset: (left: 1em, right: 1em),
    text(fill: luma(100), cont)
  )
}
