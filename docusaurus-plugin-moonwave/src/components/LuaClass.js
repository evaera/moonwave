import DocSidebar from "@theme/DocSidebar"
import Heading from "@theme/Heading"
import IconArrow from "@theme/IconArrow"
import Layout from "@theme/Layout"
import TOC from "@theme/TOC"
import clsx from "clsx"
import React, { useCallback, useEffect, useState } from "react"
import Badge from "./Badge"
import ClassMember from "./ClassMember"
import LuaFunction from "./LuaFunction"
import LuaProp from "./LuaProp"
import LuaTypeDef from "./LuaTypeDef"
import Markdown from "./Markdown"
import styles from "./styles.module.css"
import Tag from "./Tag"

const Title = Heading("h2")

const SECTIONS = [
  {
    name: "types",
    component: LuaTypeDef,
  },
  {
    name: "properties",
    component: LuaProp,
  },
  {
    name: "functions",
    component: LuaFunction,
  },
]

const capitalize = (text) => text[0].toUpperCase() + text.substring(1)

const ClassSection = ({
  luaClass,
  section,
  filter,
  component: Component,
  sourceUrl,
}) => {
  const members = luaClass[section].filter(filter || (() => true))

  if (members.length < 1) {
    return null
  }

  return (
    <>
      <Title id={section}>{capitalize(section)}</Title>
      {members.map((member, key) => (
        <ClassMember key={key} {...member} sourceUrl={sourceUrl}>
          <Component luaClassName={luaClass.name} {...member} />
        </ClassMember>
      ))}
    </>
  )
}

export default function LuaClass({
  luaClass: rawLuaClass,
  allLuaClassNames,
  options,
}) {
  const [showPrivate, setShowPrivate] = useState(false)
  const [alreadyScrolledIntoView, setAlreadyScrolledIntoView] = useState(false)

  useEffect(() => {
    setShowPrivate(localStorage.getItem("showPrivate") ? true : false)
  }, [])

  useEffect(() => {
    if (showPrivate) return

    const hash = window.location.hash.slice(1)

    if (!hash) return

    outer: for (const { name: sectionName } of SECTIONS) {
      for (const member of rawLuaClass[sectionName]) {
        if (member.name === hash) {
          setShowPrivate(true)
          break outer
        }
      }
    }
  }, [])

  useEffect(() => {
    if (!showPrivate || alreadyScrolledIntoView) return

    const element = document.getElementById(window.location.hash.slice(1))

    if (element) {
      element.scrollIntoView()
      setAlreadyScrolledIntoView(true)
    }
  }, [showPrivate])

  const [hiddenSidebar, setHiddenSidebar] = useState(false)
  const toggleSidebar = useCallback(() => {
    setHiddenSidebar(!hiddenSidebar)
  }, [hiddenSidebar])

  const luaClass = { ...rawLuaClass }

  SECTIONS.forEach((section) => {
    luaClass[section.name] = rawLuaClass[section.name]
      .filter((member) => !member.ignore)
      .filter((member) => !member.private || showPrivate)
      .sort((memberA, memberB) => {
        if (!memberA.deprecated && memberB.deprecated) {
          return -1
        } else if (memberA.deprecated && !memberB.deprecated) {
          return 1
        } else {
          return 0
        }
      })
  })

  const anyPrivateFunctions = rawLuaClass["functions"].some(
    (member) => member.private
  )

  return (
    <Layout
      title={luaClass.name}
      description={luaClass.desc}
      wrapperClassName={clsx(styles.docPageContainer)}
    >
      <div className={clsx(styles.docPage)}>
        <div
          className={clsx(styles.docSidebarContainer, {
            [styles.docSidebarContainerHidden]: hiddenSidebar,
          })}
        >
          <DocSidebar
            path={`/api/${luaClass.name}`}
            sidebar={allLuaClassNames.map((name) => ({
              type: "link",
              href: `/api/${name}`,
              label: name,
            }))}
            isHidden={hiddenSidebar}
            onCollapse={toggleSidebar}
          />
          {hiddenSidebar && (
            <div
              className={styles.collapsedDocSidebar}
              title={"Expand Sidebar"}
              aria-label={"Expand Sidebar"}
              tabIndex={0}
              role="button"
              onKeyDown={toggleSidebar}
              onClick={toggleSidebar}
            >
              <IconArrow className={styles.expandSidebarButtonIcon} />
            </div>
          )}
        </div>

        <main className={clsx(styles.docMainContainer)}>
          <div className={clsx("container padding-vert--lg")}>
            <div className="row">
              <div className="col">
                <div className={styles.docItemContainer}>
                  <article>
                    <div className={styles.member + " markdown"}>
                      <header>
                        <h1 className={styles.docTitle}>{luaClass.name}</h1>
                        <div className={clsx(styles.luaClassTags)}>
                          {luaClass.realm?.map((realm) => (
                            <Badge key={realm} label={realm} />
                          ))}
                          {luaClass.private && <Badge label="Private" />}
                          {luaClass.tags?.map((tag) => (
                            <Tag key={tag} label={tag} />
                          ))}
                        </div>

                        {anyPrivateFunctions && (
                          <label className={styles.privateToggle}>
                            <span className={styles.privateCheckboxContainer}>
                              <input
                                className={styles.privateCheckboxInternal}
                                type="checkbox"
                                name="checkbox"
                                onChange={(event) => {
                                  setShowPrivate(event.currentTarget.checked)
                                  event.currentTarget.checked
                                    ? localStorage.setItem(
                                        "showPrivate",
                                        "true"
                                      )
                                    : localStorage.removeItem("showPrivate")
                                }}
                                checked={showPrivate}
                              />
                              <span className={styles.privateCheckboxControl}>
                                <svg
                                  xmlns="http://www.w3.org/2000/svg"
                                  viewBox="0 0 24 24"
                                  aria-hidden="true"
                                  focusable="false"
                                >
                                  <path
                                    fill="none"
                                    stroke="currentColor"
                                    strokeWidth="3"
                                    d="M4 11.91l5.37 5.37L19.79 5.59"
                                  />
                                </svg>
                              </span>
                            </span>
                            <span className={styles.privateCheckboxLabel}>
                              Show Private
                            </span>
                          </label>
                        )}

                        <Markdown content={luaClass.desc} />
                      </header>

                      {SECTIONS.map((section) => (
                        <ClassSection
                          key={section.name}
                          luaClass={luaClass}
                          section={section.name}
                          component={section.component}
                          sourceUrl={options.sourceUrl}
                        />
                      ))}
                    </div>
                  </article>
                </div>
              </div>
              <div className="col col--3">
                <TOC
                  toc={SECTIONS.map((section) => ({
                    value: capitalize(section.name),
                    id: section.name,
                    children: luaClass[section.name].map((members) => ({
                      value: members.name,
                      id: members.name,
                      children: [],
                    })),
                  }))}
                />
              </div>
            </div>
            <details>
              <summary>Show raw api</summary>
              <pre
                style={{
                  maxWidth: "100%",
                  whiteSpace: "pre-wrap",
                }}
              >
                {JSON.stringify(rawLuaClass, null, 4)}
              </pre>
            </details>
          </div>
        </main>
      </div>
    </Layout>
  )
}
