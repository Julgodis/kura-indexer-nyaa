import { queryOptions } from "@tanstack/react-query"
import { ListRequest, ListResponseSchema, MirrorHealthResponseSchema, MirrorResponseSchema, ViewResponseSchema } from "./types"
import { ApiUrl } from "./url"

export const mirrorQueryOptions = queryOptions({
    queryKey: ['mirror'],
    queryFn: async () => {
        const response = await fetch(`${ApiUrl}/api/mirror`)
        return MirrorResponseSchema.parse(await response.json())
    }
})

export const listQueryOptions = (mirror: string, search: ListRequest) => {
    return queryOptions({
        queryKey: ['list', mirror, search.p, search.c, search.s, search.o, search.f, search.q],
        staleTime: 1000 * 60 * 5,
        retry: 3,
        retryDelay: 2000,
        queryFn: async () => {
            const searchParams = new URLSearchParams()
            if (search.p) searchParams.append('p', search.p.toString())
            searchParams.append('c', search.c.toString())
            searchParams.append('s', search.s.toString())
            searchParams.append('o', search.o.toString())
            searchParams.append('f', search.f.toString())
            if (search.q) searchParams.append('q', search.q.toString())

            const response = await fetch(`${ApiUrl}/api/mirror/${mirror}/list?${searchParams.toString()}`)
            return ListResponseSchema.parse(await response.json())
        }
    })
}

export const viewQueryOptions = (mirror: string, id: number) => {
    return queryOptions({
        queryKey: ['view', mirror, id],
        staleTime: 1000 * 60 * 30,
        retry: 3,
        retryDelay: 2000,
        queryFn: async () => {
            const response = await fetch(`${ApiUrl}/api/mirror/${mirror}/view/${id}`)
            return ViewResponseSchema.parse(await response.json())
        }
    })
}

export const healthQueryOptions = queryOptions({
    queryKey: ['health'],
    retry: 3,
    retryDelay: 2000,
    queryFn: async () => {
        const response = await fetch(`${ApiUrl}/api/health`)
        return MirrorHealthResponseSchema.parse(await response.json())
    }
})
