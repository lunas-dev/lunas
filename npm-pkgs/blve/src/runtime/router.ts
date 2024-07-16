import { BlveModuleExports, ComponentDeclaration } from ".";
import { routes as autoRoutes } from "blve-auto-routing-plugin";

export type ComponentLoader = () => Promise<ComponentDeclaration>;

export type Route = {
  path: string;
  component: ComponentLoader;
};

export class Router {
  routes: Route[];
  notFound: () => void;
  currentComponent: BlveModuleExports | null;
  renderingTarget!: {
    parent: HTMLElement;
    anchor: HTMLElement | null;
  };

  constructor() {
    this.routes = [];
    this.notFound = () => {};
    this.currentComponent = null;
    window.addEventListener("popstate", this.handlePopState.bind(this));
  }

  addRoute(path: string, componentLoader: ComponentLoader) {
    this.routes.push({ path, component: componentLoader });
  }

  setNotFound(notFoundHandler: () => void) {
    this.notFound = notFoundHandler;
  }

  navigate(path: string) {
    window.history.pushState({}, path, window.location.origin + path);
    this.handleRoute(path);
  }

  async handleRoute(path: string) {
    const route = this.routes.find((route) => route.path === path);
    if (route) {
      const component = await route.component();
      this.renderComponent(component);
    } else {
      this.notFound();
    }
  }

  handlePopState() {
    this.handleRoute(window.location.pathname);
  }

  renderComponent(component: ComponentDeclaration) {
    // TODO: Execute destroy function of the current component
    // if (this.currentComponent && this.currentComponent.__blve_destroy) {
    //   this.currentComponent.__blve_destroy();
    // }
    this.currentComponent = component();
    this.currentComponent.mount(document.getElementById("app")!);
  }

  initialize(parent: HTMLElement, anchor: HTMLElement | null) {
    this.renderingTarget = { parent, anchor };
    this.handleRoute(window.location.pathname);
  }

  // DO NOT EXECUTE THIS FUNCTION IN NON-VITE ENVIRONMENTS
  async initializeWithAutoRoutes(
    parent: HTMLElement,
    anchor: HTMLElement | null
  ) {
    (autoRoutes as Route[]).forEach((route) => {
      this.addRoute(route.path, route.component);
    });
    this.initialize(parent, anchor);
  }
}

export const router = new Router();
