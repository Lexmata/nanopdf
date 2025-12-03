# Branch Protection Setup Guide

This document provides step-by-step instructions for configuring branch protection rules in GitHub.

## Quick Setup

### Step 1: Access Repository Settings

1. Navigate to your repository on GitHub
2. Click **Settings** (top right)
3. Click **Branches** (left sidebar under "Code and automation")

### Step 2: Add Protection Rule for `main`

1. Click **Add branch protection rule**
2. Enter branch name pattern: `main`
3. Configure the following settings:

#### Required Settings (Minimum)

- ✅ **Require a pull request before merging**
- ✅ **Require approvals** (set to at least 1)
- ✅ **Restrict deletions** ⭐ **CRITICAL**
- ❌ **Allow force pushes** (must be DISABLED)
- ❌ **Allow deletions** (must be DISABLED) ⭐ **CRITICAL**

#### Recommended Settings

- ✅ Require status checks to pass before merging
- ✅ Require branches to be up to date before merging
- ✅ Require conversation resolution before merging
- ✅ Require signed commits
- ✅ Include administrators
- ✅ Require linear history

4. Click **Create** or **Save changes**

### Step 3: Add Protection Rule for `develop`

Repeat Step 2 with branch name pattern: `develop`

Use the same settings as for `main`.

### Step 4: Verify Configuration

1. Go to **Settings → Branches**
2. You should see two rules:
   - `main` - Protected
   - `develop` - Protected
3. Click on each rule to verify settings

## Verification

Test that protection is working:

```bash
# Try to delete main (should fail)
git push origin --delete main

# Expected error:
# remote: error: GH006: Protected branch update failed for refs/heads/main.
# remote: error: Cannot delete this protected branch
```

## GitHub CLI Setup

If you prefer automation, use GitHub CLI:

```bash
# Install GitHub CLI
# https://cli.github.com/

# Authenticate
gh auth login

# Create protection rule for main
gh api \
  --method PUT \
  /repos/:owner/:repo/branches/main/protection \
  -f required_pull_request_reviews='{"required_approving_review_count":1}' \
  -f required_status_checks=null \
  -F enforce_admins=true \
  -F restrictions=null \
  -F allow_force_pushes=false \
  -F allow_deletions=false \
  -F required_linear_history=false \
  -F lock_branch=false

# Create protection rule for develop
gh api \
  --method PUT \
  /repos/:owner/:repo/branches/develop/protection \
  -f required_pull_request_reviews='{"required_approving_review_count":1}' \
  -f required_status_checks=null \
  -F enforce_admins=true \
  -F restrictions=null \
  -F allow_force_pushes=false \
  -F allow_deletions=false \
  -F required_linear_history=false \
  -F lock_branch=false
```

## Terraform Configuration

For infrastructure-as-code approach:

```hcl
resource "github_branch_protection" "main" {
  repository_id = github_repository.repo.node_id
  pattern       = "main"

  required_pull_request_reviews {
    required_approving_review_count = 1
    dismiss_stale_reviews          = true
  }

  required_status_checks {
    strict = true
  }

  enforce_admins              = true
  allows_deletions           = false
  allows_force_pushes        = false
  require_conversation_resolution = true
  require_signed_commits      = true
  required_linear_history     = true
}

resource "github_branch_protection" "develop" {
  repository_id = github_repository.repo.node_id
  pattern       = "develop"

  required_pull_request_reviews {
    required_approving_review_count = 1
  }

  enforce_admins       = true
  allows_deletions    = false
  allows_force_pushes = false
}
```

## Troubleshooting

### "You don't have permission to edit protection rules"

- You need **Admin** permissions on the repository
- Contact the repository owner

### "Branch protection rule already exists"

- Edit the existing rule instead of creating a new one
- Click the **Edit** button next to the existing rule

### Rules not applying to administrators

- Enable **"Include administrators"** in the protection rule
- This ensures even admins follow the rules

## Status Checks

To add required status checks (CI/CD):

1. Edit branch protection rule
2. Check **"Require status checks to pass before merging"**
3. Search for and select required checks:
   - `CI / test`
   - `CI / lint`
   - `CI / build`
   - Any other workflows you want to require

Status checks must have run at least once before they appear in the list.

## FAQs

### Q: Can I delete a feature branch?

**A:** Yes! Only `main` and `develop` are protected. Feature branches can be deleted after merging.

### Q: What if I need to force-push to `main` in an emergency?

**A:** 
1. Temporarily disable protection (requires admin)
2. Make the fix
3. Re-enable protection immediately
4. Document why this was necessary

Better approach: Create a hotfix branch, fix the issue, and merge via PR.

### Q: Can I bypass these rules?

**A:** Only repository administrators can bypass rules, and even then it's not recommended. The "Include administrators" setting can prevent this.

### Q: Do these settings affect local branches?

**A:** No, these are GitHub server-side protections. They only affect push operations to GitHub.

## Related Documentation

- [Git Flow Branching Model](.cursor/rules/git-flow.mdc)
- [Branch Protection Rules](.cursor/rules/branch-protection.mdc)
- [GitHub Documentation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)

## Summary

**Critical Settings:**
- ✅ `main`: Protected, No deletions, No force pushes
- ✅ `develop`: Protected, No deletions, No force pushes

These two checkboxes are the most important:
- ❌ **Allow force pushes** - MUST BE DISABLED
- ❌ **Allow deletions** - MUST BE DISABLED

Once configured, your `main` and `develop` branches are safe from accidental deletion! 🎉

