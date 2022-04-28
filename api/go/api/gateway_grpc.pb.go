// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.2.0
// - protoc             v3.18.1
// source: api/gateway.proto

package api

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	emptypb "google.golang.org/protobuf/types/known/emptypb"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// GatewayServiceClient is the client API for GatewayService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type GatewayServiceClient interface {
	// Create creates the given gateway.
	Create(ctx context.Context, in *CreateGatewayRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	// Get returns the gateway for the given Gateway ID.
	Get(ctx context.Context, in *GetGatewayRequest, opts ...grpc.CallOption) (*GetGatewayResponse, error)
	// Update updates the given gateway.
	Update(ctx context.Context, in *UpdateGatewayRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	// Delete deletes the gateway matching the given Gateway ID.
	Delete(ctx context.Context, in *DeleteGatewayRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	// Get the list of gateways.
	List(ctx context.Context, in *ListGatewaysRequest, opts ...grpc.CallOption) (*ListGatewaysResponse, error)
	// Generate client-certificate for the gateway.
	GenerateClientCertificate(ctx context.Context, in *GenerateGatewayClientCertificateRequest, opts ...grpc.CallOption) (*GenerateGatewayClientCertificateResponse, error)
	// GetStats returns the gateway stats.
	GetStats(ctx context.Context, in *GetGatewayStatsRequest, opts ...grpc.CallOption) (*GetGatewayStatsResponse, error)
}

type gatewayServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewGatewayServiceClient(cc grpc.ClientConnInterface) GatewayServiceClient {
	return &gatewayServiceClient{cc}
}

func (c *gatewayServiceClient) Create(ctx context.Context, in *CreateGatewayRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, "/api.GatewayService/Create", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *gatewayServiceClient) Get(ctx context.Context, in *GetGatewayRequest, opts ...grpc.CallOption) (*GetGatewayResponse, error) {
	out := new(GetGatewayResponse)
	err := c.cc.Invoke(ctx, "/api.GatewayService/Get", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *gatewayServiceClient) Update(ctx context.Context, in *UpdateGatewayRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, "/api.GatewayService/Update", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *gatewayServiceClient) Delete(ctx context.Context, in *DeleteGatewayRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, "/api.GatewayService/Delete", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *gatewayServiceClient) List(ctx context.Context, in *ListGatewaysRequest, opts ...grpc.CallOption) (*ListGatewaysResponse, error) {
	out := new(ListGatewaysResponse)
	err := c.cc.Invoke(ctx, "/api.GatewayService/List", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *gatewayServiceClient) GenerateClientCertificate(ctx context.Context, in *GenerateGatewayClientCertificateRequest, opts ...grpc.CallOption) (*GenerateGatewayClientCertificateResponse, error) {
	out := new(GenerateGatewayClientCertificateResponse)
	err := c.cc.Invoke(ctx, "/api.GatewayService/GenerateClientCertificate", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *gatewayServiceClient) GetStats(ctx context.Context, in *GetGatewayStatsRequest, opts ...grpc.CallOption) (*GetGatewayStatsResponse, error) {
	out := new(GetGatewayStatsResponse)
	err := c.cc.Invoke(ctx, "/api.GatewayService/GetStats", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// GatewayServiceServer is the server API for GatewayService service.
// All implementations must embed UnimplementedGatewayServiceServer
// for forward compatibility
type GatewayServiceServer interface {
	// Create creates the given gateway.
	Create(context.Context, *CreateGatewayRequest) (*emptypb.Empty, error)
	// Get returns the gateway for the given Gateway ID.
	Get(context.Context, *GetGatewayRequest) (*GetGatewayResponse, error)
	// Update updates the given gateway.
	Update(context.Context, *UpdateGatewayRequest) (*emptypb.Empty, error)
	// Delete deletes the gateway matching the given Gateway ID.
	Delete(context.Context, *DeleteGatewayRequest) (*emptypb.Empty, error)
	// Get the list of gateways.
	List(context.Context, *ListGatewaysRequest) (*ListGatewaysResponse, error)
	// Generate client-certificate for the gateway.
	GenerateClientCertificate(context.Context, *GenerateGatewayClientCertificateRequest) (*GenerateGatewayClientCertificateResponse, error)
	// GetStats returns the gateway stats.
	GetStats(context.Context, *GetGatewayStatsRequest) (*GetGatewayStatsResponse, error)
	mustEmbedUnimplementedGatewayServiceServer()
}

// UnimplementedGatewayServiceServer must be embedded to have forward compatible implementations.
type UnimplementedGatewayServiceServer struct {
}

func (UnimplementedGatewayServiceServer) Create(context.Context, *CreateGatewayRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Create not implemented")
}
func (UnimplementedGatewayServiceServer) Get(context.Context, *GetGatewayRequest) (*GetGatewayResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Get not implemented")
}
func (UnimplementedGatewayServiceServer) Update(context.Context, *UpdateGatewayRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Update not implemented")
}
func (UnimplementedGatewayServiceServer) Delete(context.Context, *DeleteGatewayRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Delete not implemented")
}
func (UnimplementedGatewayServiceServer) List(context.Context, *ListGatewaysRequest) (*ListGatewaysResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method List not implemented")
}
func (UnimplementedGatewayServiceServer) GenerateClientCertificate(context.Context, *GenerateGatewayClientCertificateRequest) (*GenerateGatewayClientCertificateResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GenerateClientCertificate not implemented")
}
func (UnimplementedGatewayServiceServer) GetStats(context.Context, *GetGatewayStatsRequest) (*GetGatewayStatsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetStats not implemented")
}
func (UnimplementedGatewayServiceServer) mustEmbedUnimplementedGatewayServiceServer() {}

// UnsafeGatewayServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to GatewayServiceServer will
// result in compilation errors.
type UnsafeGatewayServiceServer interface {
	mustEmbedUnimplementedGatewayServiceServer()
}

func RegisterGatewayServiceServer(s grpc.ServiceRegistrar, srv GatewayServiceServer) {
	s.RegisterService(&GatewayService_ServiceDesc, srv)
}

func _GatewayService_Create_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CreateGatewayRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(GatewayServiceServer).Create(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.GatewayService/Create",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(GatewayServiceServer).Create(ctx, req.(*CreateGatewayRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _GatewayService_Get_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetGatewayRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(GatewayServiceServer).Get(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.GatewayService/Get",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(GatewayServiceServer).Get(ctx, req.(*GetGatewayRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _GatewayService_Update_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UpdateGatewayRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(GatewayServiceServer).Update(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.GatewayService/Update",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(GatewayServiceServer).Update(ctx, req.(*UpdateGatewayRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _GatewayService_Delete_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(DeleteGatewayRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(GatewayServiceServer).Delete(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.GatewayService/Delete",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(GatewayServiceServer).Delete(ctx, req.(*DeleteGatewayRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _GatewayService_List_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListGatewaysRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(GatewayServiceServer).List(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.GatewayService/List",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(GatewayServiceServer).List(ctx, req.(*ListGatewaysRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _GatewayService_GenerateClientCertificate_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GenerateGatewayClientCertificateRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(GatewayServiceServer).GenerateClientCertificate(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.GatewayService/GenerateClientCertificate",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(GatewayServiceServer).GenerateClientCertificate(ctx, req.(*GenerateGatewayClientCertificateRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _GatewayService_GetStats_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetGatewayStatsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(GatewayServiceServer).GetStats(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.GatewayService/GetStats",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(GatewayServiceServer).GetStats(ctx, req.(*GetGatewayStatsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// GatewayService_ServiceDesc is the grpc.ServiceDesc for GatewayService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var GatewayService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "api.GatewayService",
	HandlerType: (*GatewayServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Create",
			Handler:    _GatewayService_Create_Handler,
		},
		{
			MethodName: "Get",
			Handler:    _GatewayService_Get_Handler,
		},
		{
			MethodName: "Update",
			Handler:    _GatewayService_Update_Handler,
		},
		{
			MethodName: "Delete",
			Handler:    _GatewayService_Delete_Handler,
		},
		{
			MethodName: "List",
			Handler:    _GatewayService_List_Handler,
		},
		{
			MethodName: "GenerateClientCertificate",
			Handler:    _GatewayService_GenerateClientCertificate_Handler,
		},
		{
			MethodName: "GetStats",
			Handler:    _GatewayService_GetStats_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "api/gateway.proto",
}
