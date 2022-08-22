import { ThemeClassNames, useWindowSize } from "@docusaurus/theme-common"
import DocSidebar from "@theme/DocSidebar"
import Heading from "@theme/Heading"
import IconArrow from "@theme/Icon/Arrow"
import Layout from "@theme/Layout"
import TOC from "@theme/TOC"
import TOCCollapsible from "@theme/TOCCollapsible"
import clsx from "clsx"
import {
  createContext,
  default as React,
  useCallback,
  useEffect,
  useState,
} from "react"
import Admonition from "./Admonition"
import Badge from "./Badge"
import ClassMember from "./ClassMember"
import LuaFunction from "./LuaFunction"
import LuaProp from "./LuaProp"
import LuaTypeDef from "./LuaTypeDef"
import Markdown from "./Markdown"
import styles from "./styles.module.css"
import Tag from "./Tag"

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

const breakCapitalWords = (text) =>
  text
    .replace(/([A-Z])/g, " $1")
    .trim()
    .split(" ")

const ClassSection = ({
  luaClass,
  section,
  filter,
  component: Component,
  sourceUrl,
  extraTypes,
}) => {
  const members = luaClass[section].filter(filter || (() => true))

  if (members.length < 1) {
    return null
  }

  return (
    <>
      <Heading as="h2" id={section}>
        {capitalize(section)}
      </Heading>
      {members.map((member, key) => (
        <ClassMember
          key={key}
          luaClassName={luaClass.name}
          {...member}
          sourceUrl={sourceUrl}
        >
          <Component
            luaClassName={luaClass.name}
            {...member}
            extraTypes={extraTypes.get(member)}
          />
        </ClassMember>
      ))}
    </>
  )
}

const PrivateToggle = ({ showPrivate, setShowPrivate }) => (
  <label className={styles.privateToggle}>
    <span className={styles.privateCheckboxContainer}>
      <input
        className={styles.privateCheckboxInternal}
        type="checkbox"
        name="checkbox"
        onChange={(event) => {
          setShowPrivate(event.currentTarget.checked)
          event.currentTarget.checked
            ? localStorage.setItem("showPrivate", "true")
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
    <span className={styles.privateCheckboxLabel}>Show Private</span>
  </label>
)

export const TypeLinksContext = createContext()

export default function LuaClass({
  luaClass: rawLuaClass,
  sidebarClassNames,
  typeLinks,
  tocData,
  options,
}) {
  const [showPrivate, setShowPrivate] = useState(false)
  const [alreadyScrolledIntoView, setAlreadyScrolledIntoView] = useState(false)
  const anyPrivateMembers = !!tocData.find((item) => item.private)

  if (!showPrivate) {
    tocData = tocData.filter((item) => !item.private)
  }

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

  // Sort LuaClass body members
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
          if (
            memberA.function_type === "static" &&
            memberB.function_type === "method"
          ) {
            return -1
          } else if (
            memberA.function_type === "method" &&
            memberB.function_type === "static"
          ) {
            return 1
          }
          return 0
        }
      })
  })

  const extraTypes = new Map()
  const skipMembers = new Set()
  const typeOccurrences = new Map()

  for (const type of luaClass.types) {
    if (type.desc.length > 0) {
      continue
    }

    for (const fn of luaClass.functions) {
      if (
        [...fn.params, ...fn.returns].some(({ lua_type }) =>
          lua_type.includes(type.name)
        )
      ) {
        if (typeOccurrences.has(type)) {
          typeOccurrences.set(type, null)
        } else {
          typeOccurrences.set(type, fn)
        }
      }
    }
  }

  for (const [type, fn] of typeOccurrences) {
    if (!fn) {
      continue
    }

    const types = extraTypes.get(fn) || []
    extraTypes.set(fn, types)

    types.push(type)
    skipMembers.add(type)
  }

  const windowSize = useWindowSize()

  const tocMinHeadingLevel = 2 // Must be between 2 and 6 and lower or equal to the max value.
  const tocMaxHeadingLevel = 6 // Must be an integer between 2 and 6.
  const canRenderTOC = tocData && tocData.length > 0

  const renderTocDesktop =
    canRenderTOC && (windowSize === "desktop" || windowSize === "ssr")

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
            sidebar={sidebarClassNames}
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

        <main
          className={clsx(
            styles.docMainContainer,
            hiddenSidebar ? styles.docMainContainerEnhanced : ""
          )}
        >
          <div className={clsx("container padding-vert--lg")}>
            <div className="row">
              <div className={`col ${styles.docItemCol}`}>
                <div className={styles.docItemContainer}>
                  <article>
                    <TypeLinksContext.Provider value={typeLinks}>
                      <div className={styles.member + " markdown"}>
                        {canRenderTOC && (
                          <TOCCollapsible
                            toc={tocData}
                            minHeadingLevel={tocMinHeadingLevel}
                            maxHeadingLevel={tocMaxHeadingLevel}
                            className={clsx(
                              ThemeClassNames.docs.docTocMobile,
                              styles.tocMobile
                            )}
                          />
                        )}

                        <header>
                          <h1
                            className={styles.docTitle}
                            style={{
                              textDecoration: luaClass.deprecated
                                ? "line-through"
                                : "none",
                            }}
                          >
                            {breakCapitalWords(luaClass.name).map(
                              (capitalWord, i) => (
                                <span
                                  key={`${capitalWord}+${i}`}
                                  style={{ display: "inline-block" }}
                                >
                                  {capitalWord}
                                </span>
                              )
                            )}
                          </h1>
                          <div className={clsx(styles.luaClassTags)}>
                            {luaClass.realm?.map((realm) => (
                              <Badge key={realm} label={realm} />
                            ))}
                            {luaClass.private && <Badge label="Private" />}
                            {luaClass.tags?.map((tag) => (
                              <Tag key={tag} label={tag} />
                            ))}
                          </div>

                          {anyPrivateMembers && (
                            <PrivateToggle
                              showPrivate={showPrivate}
                              setShowPrivate={setShowPrivate}
                            />
                          )}

                          {luaClass.deprecated && (
                            <Admonition
                              variation="caution"
                              title={`This was deprecated in ${luaClass.deprecated.version}`}
                            >
                              {luaClass.deprecated.desc ||
                                "This item is deprecated. Do not use it for new work. "}
                            </Admonition>
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
                            filter={(member) => !skipMembers.has(member)}
                            extraTypes={extraTypes}
                          />
                        ))}
                      </div>
                    </TypeLinksContext.Provider>
                  </article>
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
              {renderTocDesktop && (
                <div className="col col--3">
                  <TOC
                    toc={tocData}
                    minHeadingLevel={tocMinHeadingLevel}
                    maxHeadingLevel={tocMaxHeadingLevel}
                  />
                </div>
              )}
            </div>
          </div>
        </main>
      </div>
    </Layout>
  )
}
