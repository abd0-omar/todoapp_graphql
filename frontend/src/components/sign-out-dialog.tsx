import { useState } from 'react'
import { useNavigate, useLocation } from '@tanstack/react-router'
import { logout } from '@/features/auth/api/auth'
import { GraphQLRequestError } from '@/lib/graphql-client'
import { useAuthStore } from '@/stores/auth-store'
import { ConfirmDialog } from '@/components/confirm-dialog'

interface SignOutDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function SignOutDialog({ open, onOpenChange }: SignOutDialogProps) {
  const navigate = useNavigate()
  const location = useLocation()
  const { auth } = useAuthStore()
  const [isSigningOut, setIsSigningOut] = useState(false)

  const handleSignOut = async () => {
    setIsSigningOut(true)
    const refresh = auth.refreshToken.trim()
    if (refresh) {
      try {
        await logout(refresh)
      } catch (error) {
        if (error instanceof GraphQLRequestError) {
          console.warn('Logout mutation failed:', error.message)
        }
      }
    }
    auth.reset()
    setIsSigningOut(false)
    onOpenChange(false)
    const currentPath = location.href
    navigate({
      to: '/sign-in',
      search: { redirect: currentPath },
      replace: true,
    })
  }

  return (
    <ConfirmDialog
      open={open}
      onOpenChange={onOpenChange}
      title='Sign out'
      desc='Are you sure you want to sign out? You will need to sign in again to access your account.'
      confirmText='Sign out'
      destructive
      handleConfirm={handleSignOut}
      isLoading={isSigningOut}
      className='sm:max-w-sm'
    />
  )
}
