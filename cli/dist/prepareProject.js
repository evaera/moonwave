import cachedir from "cachedir";
import { execSync } from "child_process";
import fs from "fs-extra";
import parseGitConfig from "parse-git-config";
import path, { dirname } from "path";
import toml from "toml";
import { fileURLToPath } from "url";
import getDocusaurusConfig from "./getDocusaurusConfig.js";
const __dirname = dirname(fileURLToPath(import.meta.url));
const TEMPLATE_PATH = path.join(__dirname, "../template");
const ROOT_PATH = path.join(TEMPLATE_PATH, "root");
const SNIP = "<!--moonwave-hide-before-this-line-->";
const INDEX_EXTS = ["html", "js", "mdx", "md"];
const COPY_FOLDERS = ["blog", "docs", "pages"];
const NO_README_TEXT = (title) => `# ${title}
This project doesn't have a README.
If it had a README.md in its root directory, you would be reading that right now.`;
const NO_GIT_REPO_TEXT = `# This project has no configured title
The site title is usually pulled from your Git repo, but no git repo could be detected.
Either set this project up as a Git repo, or configure the website title in moonwave.toml`;
function getGitRepoUrl() {
    var _a, _b, _c, _d, _e, _f;
    const gitConfig = parseGitConfig.sync();
    if (gitConfig) {
        if ((_b = (_a = gitConfig['remote "origin"']) === null || _a === void 0 ? void 0 : _a.url) === null || _b === void 0 ? void 0 : _b.includes("git@")) {
            const [, repoHostSite, repoAuthor, repoName] = (_c = gitConfig['remote "origin"']) === null || _c === void 0 ? void 0 : _c.url.replace(/\.git$/, "").match(/^git@+(.+):(.+)\/(.+)$/);
            return `https://${repoHostSite}/${repoAuthor}/${repoName}`;
        }
        else {
            return (_f = (_e = (_d = gitConfig['remote "origin"']) === null || _d === void 0 ? void 0 : _d.url) === null || _e === void 0 ? void 0 : _e.replace(/\.git$/, "")) === null || _f === void 0 ? void 0 : _f.replace(/\/\/.*@/, "//"); // Strip out http basic auth if present
        }
    }
}
function readConfig(projectDir) {
    const configPath = path.join(projectDir, "moonwave");
    if (fs.existsSync(configPath + ".toml")) {
        return toml.parse(fs.readFileSync(configPath + ".toml", { encoding: "utf-8" }));
    }
    else if (fs.existsSync(configPath + ".json")) {
        return fs.readJSONSync(configPath + ".json");
    }
    return {};
}
function getConfig(projectDir) {
    var _a, _b, _c, _d, _e, _f;
    const gitRepoUrl = getGitRepoUrl();
    const [, repoAuthor, repoName] = (gitRepoUrl === null || gitRepoUrl === void 0 ? void 0 : gitRepoUrl.match(/^https?:\/\/.+\/(.+)\/(.+)$/)) || [];
    const config = readConfig(projectDir);
    // Note: Only copying values from other places in the config should go here.
    // Default values for docusaurus.config.js belong in getDocusaurusConfig
    return {
        title: repoName,
        gitRepoUrl: gitRepoUrl,
        changelog: true,
        ...config,
        docusaurus: {
            projectName: repoName !== null && repoName !== void 0 ? repoName : undefined,
            organizationName: repoAuthor !== null && repoAuthor !== void 0 ? repoAuthor : undefined,
            title: (_b = (_a = config.title) !== null && _a !== void 0 ? _a : repoName) !== null && _b !== void 0 ? _b : "You need to configure your title",
            baseUrl: repoName ? `/${repoName}/` : "/",
            ...config.docusaurus,
        },
        navbar: {
            title: (_f = (_e = (_c = config.title) !== null && _c !== void 0 ? _c : (_d = config.docusaurus) === null || _d === void 0 ? void 0 : _d.title) !== null && _e !== void 0 ? _e : repoName) !== null && _f !== void 0 ? _f : "No Title",
            ...config.navbar,
        },
    };
}
function makeHomePage(projectDir, tempDir, config) {
    var _a, _b, _c, _d, _e;
    if (INDEX_EXTS.filter((ext) => fs.existsSync(path.join(projectDir, "pages", "index." + ext))).length === 0) {
        fs.ensureDirSync(path.join(tempDir, "pages"));
        INDEX_EXTS.forEach((ext) => fs.removeSync(path.join(tempDir, "pages", "index." + ext)));
        if ((_a = config.home) === null || _a === void 0 ? void 0 : _a.enabled) {
            const features = (_c = (_b = config.home) === null || _b === void 0 ? void 0 : _b.features) === null || _c === void 0 ? void 0 : _c.map((feature) => {
                var _a;
                if (feature.image && feature.image.startsWith("/")) {
                    feature.image = ((_a = config.docusaurus) === null || _a === void 0 ? void 0 : _a.baseUrl) + feature.image;
                }
                return feature;
            });
            let indexSource = fs
                .readFileSync(path.join(TEMPLATE_PATH, "home", "index.js"), {
                encoding: "utf-8",
            })
                .replace("/***features***/", JSON.stringify(features !== null && features !== void 0 ? features : null));
            const readmePath = path.join(projectDir, typeof ((_d = config.home) === null || _d === void 0 ? void 0 : _d.includeReadme) === "string"
                ? config.home.includeReadme
                : "README.md");
            if (((_e = config.home) === null || _e === void 0 ? void 0 : _e.includeReadme) && fs.existsSync(readmePath)) {
                fs.copyFileSync(readmePath, path.join(tempDir, "README.md"));
                let readmeContent = fs.readFileSync(readmePath, { encoding: "utf-8" });
                const snip = readmeContent.indexOf(SNIP);
                if (snip > 0) {
                    readmeContent = readmeContent.slice(snip + SNIP.length);
                }
                fs.writeFileSync(path.join(tempDir, "README.md"), readmeContent);
                indexSource = 'import README from "../README.md"\n' + indexSource;
                indexSource = indexSource.replace("{/***readme***/}", "<README />");
            }
            fs.writeFileSync(path.join(tempDir, "pages", "index.js"), indexSource);
            fs.copyFileSync(path.join(TEMPLATE_PATH, "home", "index.module.css"), path.join(tempDir, "pages", "index.module.css"));
        }
        else {
            const indexPath = path.join(tempDir, "pages", "index.md");
            const readmePath = path.join(projectDir, "README.md");
            if (fs.existsSync(readmePath)) {
                let readmeContent = fs.readFileSync(readmePath, { encoding: "utf-8" });
                const snip = readmeContent.indexOf(SNIP);
                if (snip > 0) {
                    readmeContent = readmeContent.slice(snip + SNIP.length);
                }
                fs.writeFileSync(indexPath, readmeContent);
            }
            else {
                const placeholderHomeText = config.title
                    ? NO_README_TEXT(config.title)
                    : NO_GIT_REPO_TEXT;
                fs.writeFileSync(indexPath, placeholderHomeText);
            }
        }
    }
}
function copyChangelog(projectDir, tempDir, config) {
    const changelogPath = path.join(projectDir, "CHANGELOG.md");
    const targetPath = path.join(tempDir, "pages", "CHANGELOG.md");
    if (config.changelog && fs.existsSync(changelogPath)) {
        fs.ensureDirSync(path.join(tempDir, "pages"));
        fs.copyFileSync(changelogPath, targetPath);
        return true;
    }
    else if (fs.existsSync(targetPath)) {
        fs.removeSync(targetPath);
    }
    return false;
}
function copyMoonwaveFolder(projectDir, tempDir) {
    const staticDir = path.join(projectDir, ".moonwave", "static");
    if (fs.existsSync(staticDir)) {
        fs.copySync(staticDir, path.join(tempDir, "static"));
    }
    const status = { customCssExists: false, customSidebarExists: false };
    const customCssPath = path.join(projectDir, ".moonwave", "custom.css");
    if (fs.existsSync(customCssPath)) {
        fs.copySync(customCssPath, path.join(tempDir, "src", "css", "custom.css"));
        status.customCssExists = true;
    }
    const customSidebarsPath = path.join(projectDir, ".moonwave", "sidebars.js");
    if (fs.existsSync(customSidebarsPath)) {
        fs.copySync(customSidebarsPath, path.join(tempDir, "src", "sidebars.js"));
        status.customSidebarExists = true;
    }
    return status;
}
function writeDocusaurusConfig(tempDir, params) {
    const docusaurusConfigPath = path.join(tempDir, "./docusaurus.config.js");
    const newDocusaurusConfig = "module.exports = " + JSON.stringify(getDocusaurusConfig(params), null, 2);
    if (fs.existsSync(docusaurusConfigPath) &&
        fs.readFileSync(docusaurusConfigPath, { encoding: "utf-8" }) ===
            newDocusaurusConfig) {
        return false;
    }
    else {
        fs.writeFileSync(docusaurusConfigPath, newDocusaurusConfig);
        return true;
    }
}
function copyContentFolders(projectDir, tempDir) {
    return Object.fromEntries(COPY_FOLDERS.map((folder) => {
        const folderPath = path.join(projectDir, folder);
        const targetPath = path.join(tempDir, folder);
        if (fs.existsSync(folderPath)) {
            fs.copySync(folderPath, targetPath);
            return true;
        }
        else {
            return false;
        }
    }).map((wasFound, index) => [COPY_FOLDERS[index], wasFound]));
}
function needsCompleteRebuild(tempDir) {
    if (process.env.MOONWAVE_DEV) {
        // We do fancy things to package.json in dev mode, which causes this code to always think a rebuild is needed
        return false;
    }
    if (!fs.existsSync(tempDir) ||
        !fs.existsSync(path.join(tempDir, "package.json")) ||
        !fs.existsSync(path.join(tempDir, "package-lock.json"))) {
        console.log("Moonwave: package.json or package-lock.json does not exist, rebuilding...");
        return true;
    }
    if (!fs
        .readFileSync(path.join(ROOT_PATH, "package.json"))
        .equals(fs.readFileSync(path.join(tempDir, "package.json")))) {
        console.log("Moonwave: package.json differs from cached files, rebuilding...");
        return true;
    }
    return false;
}
export function prepareProject(projectDir, options) {
    var _a, _b, _c, _d;
    const config = getConfig(projectDir);
    const folderName = (_a = projectDir.split(path.sep).slice(-1)[0]) !== null && _a !== void 0 ? _a : "unknown";
    const tempDir = path.join(cachedir("moonwave"), folderName);
    if ((options.install && fs.existsSync(tempDir)) ||
        needsCompleteRebuild(tempDir)) {
        console.log(`Deleting ${tempDir} for complete re-install`);
        fs.removeSync(tempDir);
    }
    else if (options.fresh && fs.existsSync(tempDir)) {
        for (const file of fs
            .readdirSync(tempDir)
            .filter((name) => name !== "node_modules")) {
            fs.removeSync(path.join(tempDir, file));
        }
    }
    if (!options.skipRootCopy) {
        fs.copySync(ROOT_PATH, tempDir);
        const moonwavePluginPath = process.env.MOONWAVE_PLUGIN_PATH;
        if (process.env.MOONWAVE_DEV || moonwavePluginPath) {
            console.log(`Moonwave: Using development Docusaurus plugin: ${process.env.MOONWAVE_PLUGIN_PATH || "../../docusaurus-plugin-moonwave"}`);
            const packageJsonPath = path.join(tempDir, "package.json");
            const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
            packageJson.dependencies["docusaurus-plugin-moonwave"] =
                moonwavePluginPath
                    ? path.resolve(process.cwd(), moonwavePluginPath)
                    : path.resolve(__dirname, "../../docusaurus-plugin-moonwave");
            fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));
        }
    }
    // Create home page or copy readme
    makeHomePage(projectDir, tempDir, config);
    // Copy CHANGELOG.md if it exists
    const changelogExists = copyChangelog(projectDir, tempDir, config);
    const foundFolders = copyContentFolders(projectDir, tempDir);
    const { customCssExists, customSidebarExists } = copyMoonwaveFolder(projectDir, tempDir);
    const docusaurusConfigModified = writeDocusaurusConfig(tempDir, {
        config,
        enablePlugins: foundFolders,
        customCssExists,
        customSidebarExists,
        codePaths: options.codePaths,
        binaryPath: options.binaryPath,
        changelogExists,
        projectDir,
        classOrder: (_b = config.classOrder) !== null && _b !== void 0 ? _b : [],
        apiCategories: (_c = config.apiCategories) !== null && _c !== void 0 ? _c : [],
        autoSectionPath: config.autoSectionPath,
    });
    if (!fs.existsSync(path.join(tempDir, "./node_modules")) ||
        !fs.existsSync(path.join(tempDir, "./node_modules/.bin/docusaurus"))) {
        console.log("Installing dependencies (this might take awhile)...");
        execSync("npm i", {
            cwd: tempDir,
            stdio: "inherit",
        });
    }
    return {
        docusaurusConfigModified,
        tempDir,
        projectDir,
        watchPaths: [
            typeof ((_d = config.home) === null || _d === void 0 ? void 0 : _d.includeReadme) === "string"
                ? config.home.includeReadme
                : "README.md",
            path.join(projectDir, "moonwave.toml"),
            path.join(projectDir, "moonwave.json"),
            path.join(projectDir, "CHANGELOG.md"),
            path.join(projectDir, ".moonwave/"),
            ...Object.entries(foundFolders)
                // .filter(([_folder, wasFound]) => wasFound)
                .map(([folder]) => folder)
                .map((folder) => path.join(projectDir, folder)),
        ],
    };
}
//# sourceMappingURL=prepareProject.js.map