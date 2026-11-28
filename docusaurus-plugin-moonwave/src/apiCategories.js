// Shared by the table of contents (index.js) and the class page (LuaClass.js),
// so a category link always points at a heading that exists on the page.

// The anchor for a category heading: one per section, since the same tag can
// group members in more than one section, and URL-safe, since tags may hold spaces.
export const apiCategoryId = (section, category) =>
  `${section}-${category
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")}`

// Splits a section's members into the ones outside every category and one group
// per category, in apiCategories order. A member tagged with several categories
// goes in the first, so each member is rendered once.
export function groupByApiCategory(members, apiCategories = []) {
  const placed = new Set()
  const groups = []

  for (const category of apiCategories) {
    const groupMembers = members.filter(
      (member) =>
        !placed.has(member) && member.tags && member.tags.includes(category)
    )

    if (groupMembers.length > 0) {
      groupMembers.forEach((member) => placed.add(member))
      groups.push({ category, members: groupMembers })
    }
  }

  const uncategorized = members.filter((member) => !placed.has(member))
  return { uncategorized, groups }
}
