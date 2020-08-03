provider "google" {
  project = "asia-northeast1"
}

resource "google_cloud_run_service" "blog" {
  name = "blog"
  location = "asia-northeast1"
  project = "blog-konojunya-com"

  template {
    spec {
      containers {
        image = "gcr.io/blog-konojunya-com/blog"
      }
    }
  }

  traffic {
    percent = 100
    latest_revision = true
  }
}

data "google_iam_policy" "noauth" {
  binding {
    role = "roles/run.invoker"
    members = [
      "allUsers",
    ]
  }
}

resource "google_cloud_run_service_iam_policy" "noauth" {
  location    = google_cloud_run_service.blog.location
  project     = google_cloud_run_service.blog.project
  service     = google_cloud_run_service.blog.name

  policy_data = data.google_iam_policy.noauth.policy_data
}

output "url" {
  value = "${google_cloud_run_service.blog.status[0].url}"
}
