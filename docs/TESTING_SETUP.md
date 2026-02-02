# Frontend Testing Setup - Installation Guide

## ✅ Files Created

The following test files have been created:

1. **Configuration**:
   - `vitest.config.ts` - Vitest configuration
   - `src/test/setup.ts` - Test setup with Tauri mocks

2. **Tests**:
   - `src/types/__tests__/index.test.ts` - Type definition tests (6 tests)
   - `src/components/__tests__/SafeModeToggle.test.tsx` - Component tests (10 tests)

3. **Scripts**:
   - `package.json` updated with test commands

---

## 📦 Required: Install Dependencies

You need to install the testing dependencies. Run this command:

```bash
npm install -D vitest @testing-library/react @testing-library/jest-dom @testing-library/user-event jsdom @vitejs/plugin-react
```

**Note**: There was a permission issue with npm. You may need to fix it first:

```bash
# Fix npm cache permissions
sudo chown -R $(whoami) "~/.npm"

# Then install dependencies
npm install -D vitest @testing-library/react @testing-library/jest-dom @testing-library/user-event jsdom @vitejs/plugin-react
```

---

## 🧪 Running Tests

### After Installing Dependencies

```bash
# Run all tests
npm test

# Run tests in watch mode
npm test -- --watch

# Run tests with UI
npm run test:ui

# Run with coverage report
npm run test:coverage

# Run specific test file
npm test SafeModeToggle

# Run specific test by name
npm test -- -t "should toggle safe mode"
```

---

## 📊 Test Coverage

### Type Tests (6 tests)
- ✅ Command type with required fields
- ✅ Command type with optional fields
- ✅ Command type with undefined optional fields
- ✅ Config type with safe_mode false
- ✅ Config type with safe_mode true

### Component Tests (10 tests)
- ✅ Renders toggle button
- ✅ Displays "Active Mode" when safe mode is off
- ✅ Displays "Safe Mode" when safe mode is on
- ✅ Has correct aria-label for accessibility
- ✅ Toggles safe mode when clicked
- ✅ Shows loading state during toggle
- ✅ Calls get_config on mount
- ✅ Handles error when loading config fails
- ✅ Has correct CSS classes based on state

**Total: 16 tests**

---

## 🔍 What's Being Tested

### Type Tests
- Validates TypeScript interfaces work correctly
- Ensures optional fields behave as expected
- Confirms type safety

### Component Tests
- Component rendering
- User interactions (clicking toggle)
- Loading states
- Error handling
- Accessibility features
- Tauri API integration (mocked)
- State management

---

## 🎯 Next Steps

1. **Install dependencies** (see above)
2. **Run tests**: `npm test`
3. **Add more tests** for other components:
   - `CommandForm.tsx`
   - `CommandList.tsx`
4. **Add integration tests** if needed

---

## 📝 Writing More Tests

### Example: Testing CommandForm

```typescript
import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import CommandForm from '../CommandForm';

describe('CommandForm', () => {
  it('should render form fields', () => {
    render(<CommandForm isOpen={true} onClose={() => {}} onSave={() => {}} />);
    
    expect(screen.getByLabelText(/name/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/script/i)).toBeInTheDocument();
  });
});
```

---

## ⚠️ Troubleshooting

### "Cannot find module 'vitest'"
- Run `npm install -D vitest` and other dependencies

### Tests fail with Tauri errors
- The setup file mocks Tauri API - verify `src/test/setup.ts` exists

### React rendering errors
- Make sure `@testing-library/react` is installed
- Check that `jsdom` is installed for DOM simulation

---

## 📚 Resources

- [Vitest Documentation](https://vitest.dev/)
- [React Testing Library](https://testing-library.com/docs/react-testing-library/intro/)
- [Testing Best Practices](https://kentcdodds.com/blog/common-mistakes-with-react-testing-library)
