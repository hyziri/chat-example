// Import htmx to bundle it with our code
import htmx from "htmx.org";
import "htmx-ext-preload";

// Extend the Window interface to include htmx
declare global {
  interface Window {
    htmx: typeof htmx;
  }
}

// Make htmx available globally on window
window.htmx = htmx;

// Define custom event detail types for HTMX events
interface HtmxAfterSwapDetail {
  target: HTMLElement;
  xhr: XMLHttpRequest;
  requestConfig: any;
}

interface HtmxAfterSwapEvent extends CustomEvent {
  detail: HtmxAfterSwapDetail;
}

// HTMX event handlers
document.body.addEventListener("htmx:afterSwap", function (event: Event) {
  const htmxEvent = event as HtmxAfterSwapEvent;

  // Handle SPA navigation
  // Check if the swapped element is or was #page
  if (
    htmxEvent.detail.target.id === "page" ||
    htmxEvent.detail.target.querySelector("#page") ||
    document.querySelector("#page")
  ) {
    updatePageTitle();
    // Scroll to top on navigation
    window.scrollTo({ top: 0, behavior: "smooth" });
  }
});

// Update page title from data attribute
function updatePageTitle(): void {
  // Look for data-page-title attribute in the swapped content
  const page = document.querySelector<HTMLElement>("#page");
  if (page) {
    const title = page.getAttribute("data-page-title");
    if (title) {
      document.title = title;
    }
  }
}

// Handle browser back/forward buttons
window.addEventListener("popstate", function (event: PopStateEvent): void {
  // Load content for the current URL
  const path = window.location.pathname;

  // Use HTMX to fetch the content
  window.htmx
    .ajax("get", path, {
      target: "#page",
      swap: "outerHTML",
    })
    .then(() => {
      updatePageTitle();
      window.scrollTo({ top: 0, behavior: "smooth" });
    });
});
